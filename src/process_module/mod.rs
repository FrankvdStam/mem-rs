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

mod read_write;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS32, IMAGE_NT_HEADERS64};
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY};
use crate::memory::{BaseReadWrite, ReadWrite};
use crate::process_data::ProcessData;

#[allow(dead_code)]
#[derive(Clone)]
pub struct ProcessModule
{
    process_data: Rc<RefCell<ProcessData>>,

    pub id: usize,
    pub path: String,
    pub name: String,

    pub base_address: usize,
    pub size: usize,

    pub memory: Vec<u8>,
}

impl Default for ProcessModule
{
    fn default() -> Self
    {
        ProcessModule
        {
            process_data: Rc::new(RefCell::new(ProcessData::default())),
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
    pub fn new(process_data: Rc<RefCell<ProcessData>>, id: usize, path: String, name: String, base: usize, size: usize) -> Self
    {
        ProcessModule { process_data, id, path, name, base_address: base, size, memory: Vec::new() }
    }

    pub fn dump_memory(&mut self)
    {
        let mut buffer: Vec<u8> = vec![0; self.size];
        if !self.read_memory_abs(self.base_address, &mut buffer)
        {
            return;
        }
        self.memory = buffer;

    }

    pub fn get_exports(&self) -> Vec<(String, usize)>
    {
        let mut funcs: Vec<(String, usize)> = Vec::new();

        let mut dos_header_buf: [u8; mem::size_of::<IMAGE_DOS_HEADER>()] = [0; mem::size_of::<IMAGE_DOS_HEADER>()];
        self.read_memory_abs(self.base_address, &mut dos_header_buf);
        let dos_header: IMAGE_DOS_HEADER = unsafe{ std::ptr::read(dos_header_buf.as_ptr() as *const _) };

        let export_table_address = if self.process_data.borrow().is_64_bit
        {
            let mut nt_headers_buf: [u8; mem::size_of::<IMAGE_NT_HEADERS64>()] = [0; mem::size_of::<IMAGE_NT_HEADERS64>()];
            self.read_memory_abs(self.base_address + dos_header.e_lfanew as usize, &mut nt_headers_buf);
            let nt_headers: IMAGE_NT_HEADERS64 = unsafe{ std::ptr::read(nt_headers_buf.as_ptr() as *const _)};
            nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress
        }
        else
        {
            let mut nt_headers_buf: [u8; mem::size_of::<IMAGE_NT_HEADERS32>()] = [0; mem::size_of::<IMAGE_NT_HEADERS32>()];
            self.read_memory_abs(self.base_address + dos_header.e_lfanew as usize, &mut nt_headers_buf);
            let nt_headers: IMAGE_NT_HEADERS32 =unsafe{  std::ptr::read(nt_headers_buf.as_ptr() as *const _)};
            nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress
        };

        if export_table_address == 0
        {
            return funcs;
        }

        let mut export_table_buf: [u8; mem::size_of::<IMAGE_EXPORT_DIRECTORY>()] = [0; mem::size_of::<IMAGE_EXPORT_DIRECTORY>()];
        self.read_memory_abs(self.base_address + export_table_address as usize, &mut export_table_buf);
        let export_table: IMAGE_EXPORT_DIRECTORY = unsafe{ std::ptr::read(export_table_buf.as_ptr() as *const _) };

        let name_offset_table = self.base_address + export_table.AddressOfNames as usize;
        let ordinal_table = self.base_address + export_table.AddressOfNameOrdinals as usize;
        let function_offset_table = self.base_address + export_table.AddressOfFunctions as usize;

        for i in 0..export_table.NumberOfNames {
            let mut func_name_offset_buf: [u8; mem::size_of::<u32>()] = [0; mem::size_of::<u32>()];
            self.read_memory_abs(
                name_offset_table + i as usize * mem::size_of::<u32>(),
                &mut func_name_offset_buf,
            );
            let func_name_offset: u32 = unsafe{  std::ptr::read(func_name_offset_buf.as_ptr() as *const _)};

            let func_name = read_ascii_string_generic(self, self.base_address + func_name_offset as usize);

            let mut ordinal_index_buf: [u8; mem::size_of::<u16>()] = [0; mem::size_of::<u16>()];
            self.read_memory_abs(
                ordinal_table + i as usize * mem::size_of::<u16>(),
                &mut ordinal_index_buf,
            );
            let ordinal_index: u16 = unsafe{ std::ptr::read(ordinal_index_buf.as_ptr() as *const _)};

            let mut func_offset_buf: [u8; mem::size_of::<usize>()] = [0; mem::size_of::<usize>()];
            self.read_memory_abs(
                function_offset_table + ordinal_index as usize * mem::size_of::<u32>(),
                &mut func_offset_buf,
            );
            let func_offset: u32 = unsafe{ std::ptr::read(func_offset_buf.as_ptr() as *const _)};

            let func_addr: usize = self.base_address + func_offset as usize;

            funcs.push((func_name, func_addr));
        }
        return funcs;
    }
}

fn read_ascii_string_generic<T: ReadWrite>(read_write: &T, address: usize) -> String
{
    let mut offset: usize = 0;
    let end_byte: u8 = 0x0;

    let mut output_string: String = String::from("");

    loop {
        let mut single_char_buf: [u8; 1] = [0];
        read_write.read_memory_abs(address + offset as usize, &mut single_char_buf);
        let single_char: u8 = unsafe{ std::ptr::read(single_char_buf.as_ptr() as *const _) };

        if single_char == end_byte {
            break;
        }

        output_string.push(single_char as char);

        offset += 1;

        if offset > 512 {
            panic!("String too long!");
        }
    }

    return output_string;
}