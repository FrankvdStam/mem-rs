use std::mem::size_of;
use windows::Win32::Foundation::{CloseHandle, HINSTANCE, MAX_PATH};
use windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExW};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use crate::helpers::{get_file_name_from_string, w32str_to_string};
use crate::process::Process;

impl Process
{
    pub fn get_current_process_name() -> Result<String, ()>
    {
        unsafe
            {
                let handle = GetCurrentProcess();
                let mut mod_name = [0; MAX_PATH as usize];
                if K32GetModuleFileNameExW(handle, HINSTANCE(0), &mut mod_name) != 0
                {
                    let file_path = w32str_to_string(&mod_name.to_vec());
                    let file_name = get_file_name_from_string(&file_path);
                    return Ok(file_name);
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
                        if K32GetModuleFileNameExW(handle, HINSTANCE(0), &mut mod_name) != 0
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