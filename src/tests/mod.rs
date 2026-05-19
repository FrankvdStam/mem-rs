mod game_object;
mod memory_read_write_tests;

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use test_binary::{build_test_binary, build_test_binary_once};
use windows::Win32::Foundation::{CloseHandle, MAX_PATH};
use windows::Win32::System::Memory::VirtualProtectEx;
use windows::Win32::System::ProcessStatus::K32GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use crate::helpers::{get_file_name_from_string, w32str_to_string};
use crate::memory::MemoryType;
use crate::prelude::{Process, ReadWrite};
use crate::tests::game_object::read_game_object_health;


pub struct MockProcess
{
    pub process: crate::process::Process,
    pub child_process: Option<std::process::Child>,
}

impl Drop for MockProcess
{
    fn drop(&mut self)
    {
        if self.child_process.is_some()
        {
            self.child_process.as_mut().unwrap().kill().expect("failed to kill child process");
        }
    }
}

impl MockProcess
{
    pub fn new(memory_type: MemoryType) -> Self
    {
        //calling this to make sure everything required is linked and available
        read_game_object_health();

        return match memory_type
        {
            MemoryType::Direct => {
                let self_name = Process::get_current_process_name().expect("Failed to get process name");
                let mut process = Process::new_with_memory_type(self_name.as_str(), MemoryType::Direct);
                process.refresh().expect("failed to refresh process");
                MockProcess
                {
                    process,
                    child_process: None,
                }
            },
            MemoryType::Win32Api =>
            {
                build_test_binary_once!(test_binary, "testbins");

                let test_bin_path = path_to_test_binary();

                let mut child_process = Command::new(test_bin_path)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("error running test binary");

                thread::sleep(Duration::from_millis(75));

                let mut mod_name = [0; MAX_PATH as usize];
                let mut process_name = String::new();

                if let Ok(handle) =
                    unsafe
                        {
                            OpenProcess(
                                PROCESS_QUERY_INFORMATION
                                    | PROCESS_VM_READ
                                    | PROCESS_VM_WRITE
                                    | PROCESS_VM_OPERATION,
                                false,
                                child_process.id(),
                            )
                        }
                {
                    //get file name
                    if unsafe{ K32GetModuleFileNameExW(Some(handle), None, &mut mod_name) } != 0
                    {
                        let file_path = w32str_to_string(&mod_name.to_vec());
                        process_name = get_file_name_from_string(&file_path);
                    }

                    let _ = unsafe{ CloseHandle(handle) };
                }
                if process_name.is_empty()
                {
                    panic!("Error getting test process name");
                }
                let process = crate::process::Process::new(process_name.as_str());

                MockProcess
                {
                    process,
                    child_process: Some(child_process)
                }
            }
        };
    }
}

#[test]
pub fn doit()
{
    let mut mock_process = MockProcess::new(MemoryType::Win32Api);
    mock_process.process.refresh().expect("failed to refresh");
    let game_object_pointer = mock_process.process.scan_rel("GameObject", "48 8d 05 ? ? ? ? 48 85 c0 48 8b 08", 3, 7, Vec::new()).expect("failed to scan for GameObject");
    let health = game_object_pointer.read_f32_rel(Some(0x0));
    let game_time = game_object_pointer.read_f64_rel(Some(0x8));

    assert_eq!(58.2f32, health);
    assert_eq!(94715235.165f64, game_time);
}