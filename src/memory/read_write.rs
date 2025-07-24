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

use crate::memory::base_read_write::BaseReadWrite;

pub trait ReadWrite: BaseReadWrite
{
    //==================================================================================================================================================================
    //Reading

    /// Relatively read an i8 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_i8_rel(Some(0x1234));
    /// ```
    fn read_i8_rel(&self, address: Option<usize>) -> i8
    {
        let mut buffer = [0; 1];
        self.read_memory_rel(address, &mut buffer);
        return i8::from_ne_bytes(buffer);
    }

    /// Relatively read an i32 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_i32_rel(Some(0x1234));
    /// ```
    fn read_i32_rel(&self, address: Option<usize>) -> i32
    {
        let mut buffer = [0; 4];
        self.read_memory_rel(address, &mut buffer);
        return i32::from_ne_bytes(buffer);
    }

    /// Relatively read an i64 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_i64_rel(Some(0x1234));
    /// ```
    fn read_i64_rel(&self, address: Option<usize>) -> i64
    {
        let mut buffer = [0; 8];
        self.read_memory_rel(address, &mut buffer);
        return i64::from_ne_bytes(buffer);
    }

    /// Relatively read an u8 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_u8_rel(Some(0x1234));
    /// ```
    fn read_u8_rel(&self, address: Option<usize>) -> u8
    {
        let mut buffer = [0; 1];
        self.read_memory_rel(address, &mut buffer);
        return buffer[0];
    }

    /// Relatively read an u32 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_u32_rel(Some(0x1234));
    /// ```
    fn read_u32_rel(&self, address: Option<usize>) -> u32
    {
        let mut buffer = [0; 4];
        self.read_memory_rel(address, &mut buffer);
        return u32::from_ne_bytes(buffer);
    }

    /// Relatively read an u64 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_u64_rel(Some(0x1234));
    /// ```
    fn read_u64_rel(&self, address: Option<usize>) -> u64
    {
        let mut buffer = [0; 8];
        self.read_memory_rel(address, &mut buffer);
        return u64::from_ne_bytes(buffer);
    }

    /// Relatively read an f32 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_f32_rel(Some(0x1234));
    /// ```
    fn read_f32_rel(&self, address: Option<usize>) -> f32
    {
        let mut buffer = [0; 4];
        self.read_memory_rel(address, &mut buffer);
        return f32::from_ne_bytes(buffer);
    }

    /// Relatively read an f64 from an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_f64_rel(Some(0x1234));
    /// ```
    fn read_f64_rel(&self, address: Option<usize>) -> f64
    {
        let mut buffer = [0; 8];
        self.read_memory_rel(address, &mut buffer);
        return f64::from_ne_bytes(buffer);
    }

    /// Relatively read a bool from an optional offset.
    /// A single byte is read from the resolved address, the value of the bool is true if this byte is non-zero.
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data = pointer.read_f32_rel(Some(0x1234));
    /// ```
    fn read_bool_rel(&self, address: Option<usize>) -> bool
    {
        let mut buffer = [0; 1];
        self.read_memory_rel(address, &mut buffer);
        return buffer[0] != 0;
    }

    //==================================================================================================================================================================
    //Writing

    /// Relatively write an i8 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: i8 = 10;
    /// pointer.write_i8_rel(Some(0x1234), data);
    /// ```
    fn write_i8_rel(&self, address: Option<usize>, value: i8)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    /// Relatively write an i32 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: i32 = 10;
    /// pointer.write_i32_rel(Some(0x1234), data);
    /// ```
    fn write_i32_rel(&self, address: Option<usize>, value: i32)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    /// Relatively write an i64 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: i64 = 10;
    /// pointer.write_i64_rel(Some(0x1234), data);
    /// ```
    fn write_i64_rel(&self, address: Option<usize>, value: i64)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    /// Relatively write an u8 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: u8 = 10;
    /// pointer.write_u8_rel(Some(0x1234), data);
    /// ```
    fn write_u8_rel(&self, address: Option<usize>, value: u8)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    /// Relatively write an u32 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: u32 = 10;
    /// pointer.write_u32_rel(Some(0x1234), data);
    /// ```
    fn write_u32_rel(&self, address: Option<usize>, value: u32)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    /// Relatively write an u64 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: u64 = 10;
    /// pointer.write_u64_rel(Some(0x1234), data);
    /// ```
    fn write_u64_rel(&self, address: Option<usize>, value: u64)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    /// Relatively write an f32 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: f32 = 10.0f32;
    /// pointer.write_f32_rel(Some(0x1234), data);
    /// ```
    fn write_f32_rel(&self, address: Option<usize>, value: f32)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    /// Relatively write an f64 to an optional offset
    ///
    /// # Example
    ///
    /// ```
    /// use mem_rs::prelude::*;
    ///
    /// let mut process = Process::new("name_of_process.exe");
    /// process.refresh()?;
    /// let pointer = process.create_pointer(0x1234, vec![0]);
    ///
    /// let data: f64 = 10.0f64;
    /// pointer.write_f64_rel(Some(0x1234), data);
    /// ```
    fn write_f64_rel(&self, address: Option<usize>, value: f64)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }
}