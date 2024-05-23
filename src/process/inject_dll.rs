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

use std::ffi::c_void;
use std::mem::size_of;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx};
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE, WaitForSingleObject};
use crate::helpers::{get_pcstr_from_str, get_pcwstr_from_str, vec_u16_to_u8};
use crate::prelude::*;


impl Process
{
    /// Attempts to inject a dll into the attached process using LoadLibraryW
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh().expect("Failed to attach/refresh!");
    /// process.inject_dll(r#"C:\temp\native.dll"#).expect("Failed to inject!");
    /// ```
    pub fn inject_dll(&self, dll_path: &str) -> Result<(), String>
    {
        let mut path_w32_str: Vec<u16> = dll_path.encode_utf16().collect();
        path_w32_str.push(0);

        unsafe
        {
            if self.is_attached()
            {
                let process_handle_result = OpenProcess(
                    PROCESS_CREATE_THREAD |
                        PROCESS_QUERY_INFORMATION |
                        PROCESS_VM_OPERATION |
                        PROCESS_VM_WRITE |
                        PROCESS_VM_READ, false, self.process_data.borrow().id);

                if process_handle_result.is_err()
                {
                    return Err(String::from("process handle invalid"));
                }

                let process_handle = process_handle_result.unwrap();

                //Allocate a chunk of memory inside a process and write the path to the dll in this chunk
                let allocated_dll_path_str = VirtualAllocEx(
                    process_handle,
                    None,
                    path_w32_str.len() * size_of::<u16>(),
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_READWRITE);

                self.write_memory_abs(allocated_dll_path_str as usize, &vec_u16_to_u8(&path_w32_str));

                //Get a ptr to LoadLibraryW via kernel32.dll
                let kernel32_pcwstr = get_pcwstr_from_str(&"kernel32.dll\0");

                let kernel_32_handle = GetModuleHandleW(kernel32_pcwstr);
                if kernel_32_handle.is_err()
                {
                    return  Err(String::from("failed to load module kernel32.dll"));
                }

                let load_library_w_pcstr = get_pcstr_from_str(&"LoadLibraryW\0");
                let load_library_w = GetProcAddress(kernel_32_handle.unwrap(), load_library_w_pcstr);
                if load_library_w.is_none()
                {
                    return  Err(String::from("Failed to find LoadLibraryW"));
                }

                let thread = CreateRemoteThread(
                    process_handle,
                    None,
                    0,
                    Some(*(&load_library_w.unwrap() as *const _ as *const extern "system" fn(*mut c_void) -> u32)),
                    Some(allocated_dll_path_str),
                    0,
                    None);

                if thread.is_err()
                {
                    return  Err(String::from("Failed to start remote thread"));
                }

                let _ = WaitForSingleObject(thread.unwrap(), 10000);
                let _ = VirtualFreeEx(process_handle, allocated_dll_path_str, 0, MEM_RELEASE);
            }
            return Ok(());
        }
    }
}