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
use crate::memory::{BaseReadWrite, ReadWrite};
use crate::process_data::ProcessData;


/// Represents a pointer path that is dynamically resolved each read/write operation.
/// This ensures that the pointer is always valid. Race conditions can occur and the pointer could encounter
/// a null pointer along the path. Should always be constructed via the Process struct.
///
/// # Example
///
/// ```
/// use mem_rs::prelude::*;
///
/// let mut process = Process::new("name_of_process.exe");
/// process.refresh()?;
/// let pointer = process.create_pointer(0x1234, vec![0]);
/// let data = pointer.read_u8_rel(Some(0x1234));
/// ```
pub struct Pointer
{
    process_data: Rc<RefCell<ProcessData>>,
    is_64_bit: bool,
    base_address: usize,
    offsets: Vec<usize>,
    /// Set this to true to print each memory address while resolving the pointer path.
    pub debug: bool,
}

impl Default for Pointer
{
    fn default() -> Self
    {
        Pointer
        {
            process_data: Rc::new(RefCell::new(ProcessData::default())),
            is_64_bit: true,
            base_address: 0,
            offsets: Vec::new(),
            debug: false,
        }
    }
}

impl Pointer
{
    pub(crate) fn new(process_data: Rc<RefCell<ProcessData>>, is_64_bit: bool, base_address: usize, offsets: Vec<usize>) -> Self
    {
        Pointer
        {
            process_data,
            is_64_bit,
            base_address,
            offsets,
            debug: false,
        }
    }

    /// Get the base address of this pointer, without resolving offsets.
    pub fn get_base_address(&self) -> usize
    {
        return self.base_address;
    }

    fn resolve_offsets(&self, offsets: &Vec<usize>) -> usize
    {
        let mut path = String::from(format!(" {:#010x}", self.base_address));
        let mut ptr = self.base_address;

        for i in 0..offsets.len()
        {
            let offset = offsets[i];

            //Create a copy for debug output
            let debug_copy = ptr;

            //Resolve an offset
            let address = ptr + offset;

            //Not the last offset = resolve as pointer
            if i + 1 < offsets.len()
            {
                if self.is_64_bit
                {
                    let mut buffer = [0; 8];
                    self.read_memory_abs(address, &mut buffer);
                    ptr = u64::from_ne_bytes(buffer) as usize;
                }
                else
                {
                    let mut buffer = [0; 4];
                    self.read_memory_abs(address, &mut buffer);
                    ptr = u32::from_ne_bytes(buffer) as usize;
                }

                path.push_str(format!("\n[{:#010x} + {:#010x}]: {:#010x}", debug_copy, offset, ptr).as_str());

                if ptr == 0
                {
                    if self.debug
                    {
                        println!("{}", path);
                    }
                    return 0;
                }
            }
            else
            {
                ptr = address;
                path.push_str(format!("\n{:#010x} + {:#010x}: {:#010x}", debug_copy, offset, ptr).as_str());
            }
        }
        if self.debug
        {
            println!("{}", path);
        }
        return ptr;
    }
}

impl BaseReadWrite for Pointer
{
    fn read_memory_rel(&self, offset: Option<usize>, buffer: &mut [u8]) -> bool
    {
        let mut copy = self.offsets.clone();
        if offset.is_some()
        {
            copy.push(offset.unwrap());
        }
        let address = self.resolve_offsets(&copy);
        return self.read_with_handle(self.process_data.borrow().handle, address, buffer);
    }

    fn write_memory_rel(&self, offset: Option<usize>, buffer: &[u8]) -> bool
    {
        let mut copy = self.offsets.clone();
        if offset.is_some()
        {
            copy.push(offset.unwrap());
        }
        let address = self.resolve_offsets(&copy);
        return self.write_with_handle(self.process_data.borrow().handle, address, buffer);
    }

    fn read_memory_abs(&self, address: usize, buffer: &mut [u8]) -> bool
    {
        return self.read_with_handle(self.process_data.borrow().handle, address, buffer);
    }

    fn write_memory_abs(&self, address: usize, buffer: &[u8]) -> bool
    {
        return self.write_with_handle(self.process_data.borrow().handle, address, buffer);
    }
}

impl ReadWrite for Pointer{}