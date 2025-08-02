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

use std::mem::size_of;
use windows::Win32::Foundation::{CloseHandle, FALSE, HANDLE};
use windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExW};
use windows::Win32::System::Threading::{GetExitCodeProcess, IsWow64Process, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use crate::helpers::{get_file_name_from_string, w32str_to_string};
use crate::prelude::Process;
use crate::process::STILL_ACTIVE;

impl Process
{
    /// Attempts to "attach" to a running process by name.
    /// Returns an error when the process is not running or when it has exited.
    /// Caches the main module so that pattern scans can be done against it.
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh().expect("Failed to attach/refresh!");
    /// ```
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
                    false,
                    pid,
                )
                {
                    Ok(handle) =>
                    {
                        let mut mod_name = [0; windows::Win32::Foundation::MAX_PATH as usize];

                        if K32GetModuleFileNameExW(Some(handle), None, &mut mod_name) != 0
                        {
                            let file_path = w32str_to_string(&mod_name.to_vec());
                            let file_name = get_file_name_from_string(&file_path);

                            //println!("{}", filename);

                            if self.process_data.borrow().name.to_lowercase() == file_name.to_lowercase()
                            {
                                let mut wow64 = FALSE;
                                if IsWow64Process(handle, &mut wow64).is_ok()
                                {
                                    let mut main_module = Process::get_process_modules(handle, &self.process_data).remove(0);
                                    main_module.dump_memory(handle);

                                    let mut process_data = self.process_data.borrow_mut();

                                    process_data.id = pid;
                                    process_data.handle = handle;
                                    process_data.is_64_bit = !wow64.as_bool();
                                    process_data.filename = file_name;
                                    process_data.path = file_path;
                                    process_data.attached = true;

                                    self.main_module = Some(main_module);

                                    return Ok(());
                                }
                            }
                        }

                        let _ = CloseHandle(handle);
                    }
                    _ => {},
                }
            }
            return Err(String::from("Process not running"));
        }
    }
}