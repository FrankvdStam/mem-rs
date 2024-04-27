// This file is part of the mem-rs distribution (https://github.com/FrankvdStam/mem-rs).
// Copyright (c) 2022 Frank van der Stam.
// https://github.com/FrankvdStam/mem-rs/blob/main/LICENSE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::cell::RefCell;
use std::ffi::c_void;
use std::mem::size_of;
use std::path::Path;
use std::rc::Rc;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::{BOOL, CloseHandle, HANDLE, HINSTANCE, MAX_PATH};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx};
use windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32EnumProcessModules, K32GetModuleFileNameExA, K32GetModuleFileNameExW, K32GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::{GetCurrentProcess, GetExitCodeProcess, OpenProcess, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use crate::helpers::{scan, to_pattern};
use crate::pointer::Pointer;
use crate::process_data::ProcessData;
use crate::process_module::ProcessModule;
use crate::read_write::{BaseReadWrite, ReadWrite};

#[link(name = "kernel32")]
extern "stdcall"
{
    fn GetProcAddress(h_module: *const c_void, lp_proc_name: *const u8) -> *const c_void;
    fn GetModuleHandleA(lp_module_name: *const u8) -> HINSTANCE;
    fn CreateRemoteThread(h_process: *const c_void, lp_thread_attributes: *const c_void, dw_stack_size: u32, lp_start_address: *const c_void, lp_parameter: *const c_void, dw_creation_flags: u32, lp_thread_id: *const c_void) -> *const c_void;
    fn WaitForSingleObject(handle: *const c_void, dw_milliseconds: u32) -> u32;
}

const STILL_ACTIVE: u32 = 259;


pub struct Process
{
    process_data: Rc<RefCell<ProcessData>>
}

impl Process
{
    ///Create a new process where name is the name of the executable
    pub fn new(name: &str) -> Self
    {
        Process
        {
            process_data: Rc::new(RefCell::new(ProcessData
            {
                name: String::from(name),
                attached: false,
                id: 0,
                handle: HANDLE::default(),
                filename: String::new(),
                path: String::new(),
                main_module: ProcessModule::default(),
                modules: Vec::new(),
            }))
        }
    }

    pub fn is_attached(&self) -> bool {return self.process_data.borrow().attached;}

    pub fn scan_abs(&self, error_name: &str, pattern: &str, scan_offset: usize, pointer_offsets: Vec<usize>) -> Result<Pointer, String>
    {
        let byte_pattern = to_pattern(pattern);
        let scan_result = scan(&self.process_data.borrow().main_module.memory, &byte_pattern);
        if scan_result.is_none()
        {
            return Err(String::from(format!("Scan failed: {}", error_name)));
        }

        let mut address = scan_result.unwrap();
        address += self.process_data.borrow().main_module.base_address;
        address += scan_offset;
        return Ok(Pointer::new(self.process_data.clone(), true, address, pointer_offsets));
    }

    pub fn scan_rel(&self, error_name: &str, pattern: &str, scan_offset: usize, instruction_size: usize, pointer_offsets: Vec<usize>) -> Result<Pointer, String>
    {
        let byte_pattern = to_pattern(pattern);
        let scan_result = scan(&self.process_data.borrow().main_module.memory, &byte_pattern);
        if scan_result.is_none()
        {
            return Err(String::from(format!("Scan failed: {}", error_name)));
        }

        let address = scan_result.unwrap();
        let address_value = self.read_u32_rel(Some(address + scan_offset));
        let result = self.process_data.borrow().main_module.base_address + address + instruction_size + address_value as usize; //Relative jump

        return Ok(Pointer::new(self.process_data.clone(), true, result, pointer_offsets));
    }

    pub fn inject_dll(&self, dll_path: &str)
    {
        let mut temp = dll_path.to_string();
        temp.push('\0');
        let dll_path_null_terminated = temp.as_str();

        unsafe
        {
            if self.is_attached()
            {
                let process_handle = OpenProcess(
                    PROCESS_CREATE_THREAD |
                    PROCESS_QUERY_INFORMATION |
                    PROCESS_VM_OPERATION |
                    PROCESS_VM_WRITE |
                    PROCESS_VM_READ, false, self.process_data.borrow().id).unwrap();

                //println!("{:?}", process_handle);

                //Allocate a chunk of memory inside a process and write the path to the dll in this chunk
                let allocated_dll_path_str = VirtualAllocEx(
                    process_handle,
                    None,
                    dll_path_null_terminated.len() * size_of::<u8>(),
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_READWRITE);

                //println!("{:?}", allocated_dll_path_str as *const ());

                self.write_memory_abs(
                    allocated_dll_path_str as usize,
                    dll_path_null_terminated.as_bytes()
                );

                //Get a ptr to LoadLibraryA via kernel32.dll
                //let kernel32_str = "KERNEL32.DLL\0";
                let kernel32_str = "kernel32.dll\0";
                let load_library_a_str = "LoadLibraryA\0";

                let kernel_32_handle = GetModuleHandleA(kernel32_str.as_ptr());
                let load_library_a = GetProcAddress(kernel_32_handle.0 as *const c_void, load_library_a_str.as_ptr());

                //Call LoadLibraryA with our allocated and wait for it to return 10 seconds
                let thread = CreateRemoteThread(
                    process_handle.0 as *const c_void,
                    0 as *const c_void,
                    0,
                    load_library_a as *const c_void,
                    allocated_dll_path_str,
                    0,
                    0 as *const c_void);
                WaitForSingleObject(thread, 10000);

                VirtualFreeEx(process_handle, allocated_dll_path_str, 0, MEM_RELEASE);
            }
        }
    }


    ///Cling to a running process
    pub fn refresh(&mut self) -> Result<(), String>
    {
        unsafe
        {
            //Check if a previously attached process has exited
            let mut lp_exit_code: u32 = 0;
            if self.process_data.borrow().attached && (!GetExitCodeProcess(self.process_data.borrow().handle, &mut lp_exit_code).is_ok() || lp_exit_code != STILL_ACTIVE)
            {
                let mut process_data = self.process_data.borrow_mut();

                process_data.attached = false;
                process_data.id = 0;
                process_data.handle = HANDLE::default();
                process_data.filename = String::new();
                process_data.path = String::new();
                process_data.main_module = ProcessModule::default();
                process_data.modules = Vec::new();

                return Err(String::from("Process exited"));
            }

            if self.process_data.borrow().attached
            {
                return Ok(());
            }

            //Look for a running process with the correct name and attach to it
            let mut process_ids = [0u32; 2048];
            let mut out_size = 0;

            if !K32EnumProcesses(process_ids.as_mut_ptr(), (process_ids.len() * size_of::<u32>()) as u32, &mut out_size).as_bool()
            {
                return Err(String::from("Failed to get running processes"));
            }

            let count = out_size as usize / std::mem::size_of::<u32>();
            for i in 0..count
            {
                let pid = process_ids[i];

                match OpenProcess(
                    PROCESS_QUERY_INFORMATION
                        | PROCESS_VM_READ
                        | PROCESS_VM_WRITE
                        | PROCESS_VM_OPERATION,
                    BOOL(0),
                    pid,
                )
                {
                    Ok(handle) =>
                    {
                        let mut mod_name = [0; windows::Win32::Foundation::MAX_PATH as usize];

                        if K32GetModuleFileNameExA(handle, HINSTANCE(0), &mut mod_name) != 0
                        {
                            let len  = mod_name.iter().position(|&r| r == 0).unwrap();
                            let path = String::from_utf8(mod_name[0..len].iter().map(|&c| c as u8).collect()).unwrap();
                            let filename = String::from(Path::new(&path).file_name().unwrap().to_str().unwrap());

                            //println!("{}", filename);

                            if self.process_data.borrow().name.to_lowercase() == filename.to_lowercase()
                            {
                                let mut modules = Process::get_process_modules(handle);

                                let mut process_data = self.process_data.borrow_mut();

                                process_data.id = pid;
                                process_data.handle = handle;
                                process_data.filename = filename;
                                process_data.path = path;
                                process_data.attached = true;
                                process_data.main_module = modules.remove(0);
                                process_data.main_module.dump_memory(handle);
                                process_data.modules = modules;

                                return Ok(());
                            }
                        }

                        CloseHandle(handle);
                    }
                    _ => {},
                }
            }
            return Err(String::from("Process not running"));
        }
    }

    fn get_process_modules(process_handle: HANDLE) -> Vec<ProcessModule>
    {
        unsafe
        {
            let mut result = Vec::new();

            //Get amount of hmodules in current process
            let mut required_size: u32 = 0;
            let _ = K32EnumProcessModules(process_handle, 0 as *mut HMODULE, 0, &mut required_size);
            let size = (required_size / size_of::<HINSTANCE>() as u32) as u32;

            //Get modules
            let mut modules: Vec<HMODULE> = vec![HMODULE(0); size as usize];
            let _ = K32EnumProcessModules(process_handle, modules.as_mut_ptr(), required_size.clone(), &mut required_size).unwrap();

            for i in 0..modules.len()
            {
                let mut mod_name = [0; MAX_PATH as usize];

                if K32GetModuleFileNameExA(process_handle, modules[i as usize], &mut mod_name) != 0
                {
                    let len  = mod_name.iter().position(|&r| r == 0).unwrap();
                    let file_path = String::from_utf8(mod_name[0..len].iter().map(|&c| c as u8).collect()).unwrap();
                    let file_name = Path::new(&file_path).file_name().unwrap().to_os_string().into_string().unwrap();

                    let mut info: MODULEINFO = MODULEINFO
                    {
                        lpBaseOfDll: 0 as *mut c_void,
                        SizeOfImage: 0,
                        EntryPoint: 0 as *mut c_void,
                    };

                    if K32GetModuleInformation(process_handle, modules[i as usize], &mut info, size_of::<MODULEINFO>() as u32).as_bool()
                    {
                        let module_base = info.lpBaseOfDll as usize;
                        let module_size = info.SizeOfImage as usize;
                        result.push(ProcessModule::new(modules[i as usize].0 as usize, file_path, file_name, module_base, module_size));
                    }
                }
            }
            return result;
        }
    }

    pub fn get_current_process_name() -> Result<String, ()>
    {
        unsafe
        {
            let handle = GetCurrentProcess();
            let mut mod_name = [0; MAX_PATH as usize];
            if K32GetModuleFileNameExA(handle, HINSTANCE(0), &mut mod_name) != 0
            {
                let len  = mod_name.iter().position(|&r| r == 0).unwrap();
                let path = String::from_utf8(mod_name[0..len].iter().map(|&c| c as u8).collect()).unwrap();
                let filename = String::from(Path::new(&path).file_name().unwrap().to_str().unwrap());
                return Ok(filename);
            }
            Err(())
        }
    }

    pub fn get_running_process_names() -> Vec<String>
    {
        unsafe
        {
            let mut process_names = Vec::new();
            let mut process_ids = [0u32; 2048];
            let mut bytes_needed = 0u32;
            K32EnumProcesses(process_ids.as_mut_ptr(), (process_ids.len() * size_of::<u32>()) as u32, &mut bytes_needed);
            let count = bytes_needed as usize / std::mem::size_of::<u32>();

            for i in 0..count
            {
                let pid = process_ids[i];

                let mut mod_name = [0; MAX_PATH as usize];

                if let Ok(handle) = OpenProcess(
                    PROCESS_QUERY_INFORMATION
                        | PROCESS_VM_READ
                        | PROCESS_VM_WRITE
                        | PROCESS_VM_OPERATION,
                    false,
                    pid,
                )
                {
                    if K32GetModuleFileNameExA(handle, HINSTANCE(0), &mut mod_name) != 0
                    {
                        let len  = mod_name.iter().position(|&r| r == 0).unwrap();
                        let path = String::from_utf8(mod_name[0..len].iter().map(|&c| c as u8).collect()).unwrap();
                        let filename = String::from(Path::new(&path).file_name().unwrap().to_str().unwrap());
                        process_names.push(filename);
                    }
                    CloseHandle(handle);
                }
            }
            return process_names;
        }
    }
}

impl BaseReadWrite for Process
{
    fn read_memory_rel(&self, offset: Option<usize>, buffer: &mut [u8]) -> bool
    {
        let mut address = self.process_data.borrow().main_module.base_address;
        if offset.is_some()
        {
            address += offset.unwrap();
        }
        return self.read_with_handle(self.process_data.borrow().handle, address, buffer);
    }

    fn write_memory_rel(&self, offset: Option<usize>, buffer: &[u8]) -> bool
    {
        let mut address = self.process_data.borrow().main_module.base_address;
        if offset.is_some()
        {
            address += offset.unwrap();
        }
        return self.write_with_handle(self.process_data.borrow().handle, address, buffer);
    }

    fn read_memory_abs(&self, address: usize, buffer: &mut [u8]) -> bool
    {
        return self.read_with_handle(self.process_data.borrow().handle, address, buffer);
    }

    fn write_memory_abs(&self, address: usize, buffer: &[u8]) -> bool
    {
        return self.write_with_handle(self.process_data.borrow().handle, address, buffer);
    }
}

impl ReadWrite for Process{}