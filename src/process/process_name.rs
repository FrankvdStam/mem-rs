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
use windows::Win32::Foundation::{CloseHandle, HMODULE, MAX_PATH};
use windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExW};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use crate::helpers::{get_file_name_from_string, w32str_to_string};
use crate::process::Process;

impl Process
{
    /// Returns the current process name, in which this very code is running.
    /// Does NOT return the name of the target attachment process.
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let name = Process::get_current_process_name()?;
    /// ```
    pub fn get_current_process_name() -> Result<String, ()>
    {
        unsafe
        {
            let handle = GetCurrentProcess();
            let mut mod_name = [0; MAX_PATH as usize];
            if K32GetModuleFileNameExW(handle, HMODULE::default(), &mut mod_name) != 0
            {
                let file_path = w32str_to_string(&mod_name.to_vec());
                let file_name = get_file_name_from_string(&file_path);
                return Ok(file_name);
            }
            Err(())
        }
    }

    /// Returns all the processes that are currently running
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let names = Process::get_running_process_names();
    /// ```
    pub fn get_running_process_names() -> Vec<String>
    {
        unsafe
        {
            let mut process_names = Vec::new();
            let mut process_ids = [0u32; 2048];
            let mut bytes_needed = 0u32;
            let _ = K32EnumProcesses(process_ids.as_mut_ptr(), (process_ids.len() * size_of::<u32>()) as u32, &mut bytes_needed);
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
                    if K32GetModuleFileNameExW(handle, HMODULE::default(), &mut mod_name) != 0
                    {
                        let file_path = w32str_to_string(&mod_name.to_vec());
                        let file_name = get_file_name_from_string(&file_path);
                        process_names.push(file_name);
                    }
                    let _ = CloseHandle(handle);
                }
            }
            return process_names;
        }
    }
}