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
use windows::Win32::Foundation::{HANDLE, HINSTANCE, HMODULE, MAX_PATH};
use windows::Win32::System::ProcessStatus::{K32EnumProcessModules, K32GetModuleFileNameExW, K32GetModuleInformation, MODULEINFO};
use crate::helpers::{get_file_name_from_string, w32str_to_string};
use crate::process::Process;
use crate::process_module::ProcessModule;

impl Process
{
    pub(crate) fn get_process_modules(process_handle: HANDLE) -> Vec<ProcessModule>
    {
        unsafe
            {
                let mut result = Vec::new();

                //Get amount of hmodules in current process
                let mut required_size: u32 = 0;
                let _ = K32EnumProcessModules(process_handle, 0 as *mut HMODULE, 0, &mut required_size);
                let size = (required_size / size_of::<HINSTANCE>() as u32) as u32;

                //Get modules
                let mut modules: Vec<HMODULE> = vec![HMODULE::default(); size as usize];
                let _ = K32EnumProcessModules(process_handle, modules.as_mut_ptr(), required_size.clone(), &mut required_size).unwrap();

                for i in 0..modules.len()
                {
                    let mut mod_name = [0; MAX_PATH as usize];

                    if K32GetModuleFileNameExW(Some(process_handle), Some(modules[i as usize]), &mut mod_name) != 0
                    {
                        let file_path = w32str_to_string(&mod_name.to_vec());
                        let file_name = get_file_name_from_string(&file_path);

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
}