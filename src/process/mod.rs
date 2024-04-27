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
}

