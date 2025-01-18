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
use std::rc::Rc;

use windows::Win32::Foundation::HANDLE;

use crate::process_data::ProcessData;
use crate::process_module::ProcessModule;
mod inject_dll;
mod scanning;
mod read_write;
mod refresh;
mod process_modules;
mod process_name;

const STILL_ACTIVE: u32 = 259;

/// Wraps a native process and allows memory access/manipulation
///
/// # Examples
///
/// ```
/// use mem_rs::prelude::*;
///
/// let mut process = Process::new("name_of_process.exe");
/// if process.refresh().is_ok()
/// {
///     process.write_memory_abs(0x1234, &u32::to_ne_bytes(10));
///     let result = process.read_u32_rel(Some(0x1234));
///     println!("Result: {}", result);
/// }
/// ```
pub struct Process
{
    process_data: Rc<RefCell<ProcessData>>
}

impl Process
{
    /// Creates a new process based on the process name.
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// ```
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

    /// Returns if the process is "attached" and can be read/written from/to
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// //returns false
    /// let not_attached = process.is_attached();
    ///
    /// //refreshing the process will cause it to become attached
    /// process.refresh().unwrap();
    ///
    /// //if name_of_process.exe is running, will return true
    /// let attached = process.is_attached();
    /// ```
    pub fn is_attached(&self) -> bool {return self.process_data.borrow().attached;}

    /// Returns file path of the processes' executable
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    /// 
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh().unwrap();
    /// 
    /// println!("{}", process.get_path());
    /// ```
    pub fn get_path(&self) -> String {return self.process_data.borrow().path.clone();}

    /// Returns modules of a process
    ///
    /// # Examples
    /// ```
    /// use mem_rs::prelude::*;
    /// 
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh().unwrap();
    /// 
    /// let process_modules = process.get_modules();
    /// 
    /// for process_module in process_modules {
    ///     println!("{}", process_module.name);
    /// }
    /// ```
    pub fn get_modules(&self) -> Vec<ProcessModule> {return self.process_data.borrow().modules.clone();}
}
