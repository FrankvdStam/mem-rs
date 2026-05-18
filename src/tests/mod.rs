use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use test_binary::{build_test_binary, build_test_binary_once};
use windows::Win32::Foundation::{CloseHandle, MAX_PATH};
use windows::Win32::System::ProcessStatus::K32GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use crate::helpers::{get_file_name_from_string, w32str_to_string};
use crate::prelude::ReadWrite;

build_test_binary_once!(test_binary, "testbins");

pub struct MockProcess
{
    pub process: crate::process::Process,
    pub child_process: std::process::Child,
}

impl Drop for MockProcess
{
    fn drop(&mut self)
    {
        self.child_process.kill().expect("failed to kill child process");
    }
}

impl MockProcess
{
    pub fn new() -> Self
    {
        let test_bin_path = path_to_test_binary();

        let mut child_process = Command::new(test_bin_path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("error running test binary");


        // //it takes some time for the process to properly startup and for it to load win32 modules
        // let iter_max = 100;
        // let mut iter = 0;
        // let mut stdout_buffer_str = String::new();
        //
        // //wait for output stream to be initialized
        // while child_process.stdout.is_none() && iter < iter_max
        // {
        //     iter += 1;
        //     println!("{}", iter);
        //     thread::sleep(Duration::from_millis(1));
        // }
        //
        // let output_stream = &mut child_process.stdout.take().expect("no stdout stream");
        //
        // iter = 0;
        // while stdout_buffer_str != "asd" && iter < iter_max
        // {
        //     let mut buff = Vec::new();
        //    let res = output_stream.read(&mut buff);
        //
        //     iter += 1;
        //     println!("{}", stdout_buffer_str);
        //     println!("{}", iter);
        //     thread::sleep(Duration::from_millis(1));
        // }

        thread::sleep(Duration::from_millis(50));


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

        return MockProcess
        {
            process,
            child_process
        };
    }
}

#[test]
pub fn doit()
{
    let mut mock_process = MockProcess::new();
    mock_process.process.refresh().expect("failed to refresh");
    let game_object_pointer = mock_process.process.scan_rel("GameObject", "48 8d 05 ? ? ? ? 48 85 c0 48 8b 08", 3, 7, Vec::new()).expect("failed to scan for GameObject");
    let health = game_object_pointer.read_f32_rel(Some(0x0));
    let game_time = game_object_pointer.read_f64_rel(Some(0x8));

    assert_eq!(58.2f32, health);
    assert_eq!(94715235.165f64, game_time);
}