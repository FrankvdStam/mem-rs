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
use windows::Win32::Foundation::{HANDLE};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

pub struct ProcessModule
{
    pub id: usize,
    pub path: String,
    pub name: String,

    pub base_address: usize,
    pub size: usize,

    pub memory: Vec<u8>,
}

impl Default for ProcessModule
{
    fn default() -> Self {
        ProcessModule
        {
            id: 0,
            path: String::new(),
            name: String::new(),
            base_address: 0,
            size: 0,
            memory: Vec::new(),
        }
    }
}

impl ProcessModule
{
    pub fn new(id: usize, path: String, name: String, base: usize, size: usize) -> Self
    {
        ProcessModule { id, path, name, base_address: base, size, memory: Vec::new() }
    }

    pub fn dump_memory(&mut self, process_handle: HANDLE)
    {
        unsafe
        {
            let mut buffer: Vec<u8> = vec![0; self.size];
            let mut read_bytes = 0;
            if !ReadProcessMemory(process_handle as HANDLE, self.base_address as *mut c_void, buffer.as_mut_ptr() as *mut c_void, buffer.capacity(), &mut read_bytes).as_bool()
            {
                return;
            }
            self.memory = buffer;
        }
    }
}