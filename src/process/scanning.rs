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

use crate::helpers::{scan, to_pattern};
use crate::pointer::Pointer;
use crate::prelude::*;

impl Process
{
    /// Does an absolute scan (for x86 targets or for process code) where the target pointer is absolute
    /// Takes a list of offsets to create pointer jumps down a bigger complex structure.
    /// Pointers implement memory reading and writing.
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.scan_abs("Error message", "56 8B F1 8B 46 1C 50 A1 ? ? ? ? 32 C9", 8, vec![0, 0, 0])?;
    /// ```
    pub fn scan_abs(&self, error_name: &str, pattern: &str, scan_offset: usize, pointer_offsets: Vec<usize>) -> Result<Pointer, String>
    {
        let byte_pattern = to_pattern(pattern);
        let scan_result = scan(&self.get_main_module().memory, &byte_pattern);
        if scan_result.is_none()
        {
            return Err(String::from(format!("Scan failed: {}", error_name)));
        }

        let mut address = scan_result.unwrap();
        address += self.get_main_module().base_address;
        address += scan_offset;
        return Ok(Pointer::new(self.process_data.clone(), true, address, pointer_offsets));
    }

    /// Does a relative scan (for x64 targets) where the target pointer is located relative to instruction's
    /// size and location.
    /// Takes a list of offsets to create pointer jumps down a bigger complex structure.
    /// Pointers implement memory reading and writing.
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.scan_rel("Error message", "48 8b 05 ? ? ? ? 48 8b 50 10 48 89 54 24 60", 3, 7, vec![0])?;
    /// ```
    pub fn scan_rel(&self, error_name: &str, pattern: &str, scan_offset: usize, instruction_size: usize, pointer_offsets: Vec<usize>) -> Result<Pointer, String>
    {
        let byte_pattern = to_pattern(pattern);
        let scan_result = scan(&self.get_main_module().memory, &byte_pattern);
        if scan_result.is_none()
        {
            return Err(String::from(format!("Scan failed: {}", error_name)));
        }

        let address = scan_result.unwrap();
        let address_value = self.read_u32_rel(Some(address + scan_offset));
        let result = self.get_main_module().base_address + address + instruction_size + address_value as usize; //Relative jump

        return Ok(Pointer::new(self.process_data.clone(), true, result, pointer_offsets));
    }

    /// Create a pointer without scanning from an absolute address and a list of offsets.
    /// For special use cases where an address might be the result of some calculation.
    ///
    ///  let network = vanilla.process.create_pointer(network_ptr as usize, vec![0xc, 0x6c978])
    ///
    /// # Examples
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let magic_address = 0x1234;
    /// let pointer = process.create_pointer(magic_address, vec![0xc, 0x10]);
    /// ```
    pub fn create_pointer(&self, address: usize, pointer_offsets: Vec<usize>) -> Pointer
    {
        return Pointer::new(self.process_data.clone(), self.is_64_bit(), address, pointer_offsets);
    }
}