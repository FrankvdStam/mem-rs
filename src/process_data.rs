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

use windows::Win32::Foundation::HANDLE;
use crate::process_module::ProcessModule;

pub struct ProcessData
{
    pub attached: bool,
    pub name: String,

    pub filename: String,
    pub path: String,

    pub id: u32,
    pub handle: HANDLE,

    pub main_module: ProcessModule,
    pub modules: Vec<ProcessModule>,
}

impl Default for ProcessData
{
    fn default() -> Self
    {
        ProcessData
        {
            name: String::new(),
            attached: false,
            id: 0,
            handle: HANDLE::default(),
            filename: String::new(),
            path: String::new(),
            main_module: ProcessModule::default(),
            modules: Vec::new(),
        }
    }
}