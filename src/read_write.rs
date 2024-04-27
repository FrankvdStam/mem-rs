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
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ ReadProcessMemory, WriteProcessMemory};

pub trait BaseReadWrite
{
    ///Read memory relative to the object's location in memory
    fn read_memory_rel(&self, offset: Option<usize>, buffer: &mut [u8]) -> bool;

    ///Write memory relative to the object's location in memory
    fn write_memory_rel(&self, offset: Option<usize>, buffer: &[u8]) -> bool;

    ///Read memory at absolute address
    fn read_memory_abs(&self, address: usize, buffer: &mut [u8]) -> bool;

    ///Write memory at absolute address
    fn write_memory_abs(&self, address: usize, buffer: &[u8]) -> bool;

    ///Read memory into a buffer from a process handle
    fn read_with_handle(&self, handle: HANDLE, address: usize, buffer: &mut [u8]) -> bool
    {
        let mut read_bytes = 0;
        if unsafe {ReadProcessMemory(handle, address as *mut c_void, buffer.as_mut_ptr() as *mut c_void, buffer.len(), Some(&mut read_bytes)).is_err() }
        {
            return false;
        }
        return read_bytes == buffer.len();
    }

    ///Write from a buffer ino memory from a process handle
    fn write_with_handle(&self, handle: HANDLE, address: usize, buffer: &[u8]) -> bool
    {
        let mut wrote_bytes = 0;
        if unsafe { WriteProcessMemory(handle, address as *mut c_void, buffer.as_ptr() as *mut c_void, buffer.len(), Some(&mut wrote_bytes)).is_err() }
        {
            return false;
        }
        return wrote_bytes == buffer.len();
    }
}

pub trait ReadWrite: BaseReadWrite
{
    ///==================================================================================================================================================================
    /// Reading

    ///Read an i8 from the given address
    fn read_i8_rel(&self, address: Option<usize>) -> i8
    {
        let mut buffer = [0; 1];
        self.read_memory_rel(address, &mut buffer);
        return i8::from_ne_bytes(buffer);
    }

    ///Read an i32 from the given address
    fn read_i32_rel(&self, address: Option<usize>) -> i32
    {
        let mut buffer = [0; 4];
        self.read_memory_rel(address, &mut buffer);
        return i32::from_ne_bytes(buffer);
    }

    ///Read an i64 from the given address
    fn read_i64_rel(&self, address: Option<usize>) -> i64
    {
        let mut buffer = [0; 8];
        self.read_memory_rel(address, &mut buffer);
        return i64::from_ne_bytes(buffer);
    }

    ///Read an u8 from the given address
    fn read_u8_rel(&self, address: Option<usize>) -> u8
    {
        let mut buffer = [0; 1];
        self.read_memory_rel(address, &mut buffer);
        return buffer[0];
    }

    ///Read an u32 from the given address
    fn read_u32_rel(&self, address: Option<usize>) -> u32
    {
        let mut buffer = [0; 4];
        self.read_memory_rel(address, &mut buffer);
        return u32::from_ne_bytes(buffer);
    }

    ///Read an u64 from the given address
    fn read_u64_rel(&self, address: Option<usize>) -> u64
    {
        let mut buffer = [0; 8];
        self.read_memory_rel(address, &mut buffer);
        return u64::from_ne_bytes(buffer);
    }

    ///Read an f32 from the given address
    fn read_f32_rel(&self, address: Option<usize>) -> f32
    {
        let mut buffer = [0; 4];
        self.read_memory_rel(address, &mut buffer);
        return f32::from_ne_bytes(buffer);
    }

    ///Read an f64 from the given address
    fn read_f64_rel(&self, address: Option<usize>) -> f64
    {
        let mut buffer = [0; 8];
        self.read_memory_rel(address, &mut buffer);
        return f64::from_ne_bytes(buffer);
    }

    ///Read a bool from the given address
    fn read_bool_rel(&self, address: Option<usize>) -> bool
    {
        let mut buffer = [0; 1];
        self.read_memory_rel(address, &mut buffer);
        return buffer[0] != 0;
    }

    ///==================================================================================================================================================================
    /// Writing

    ///Write an i8 to the given address
    fn write_i8_rel(&self, address: Option<usize>, value: i8)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    ///Write an i32 to the given address
    fn write_i32_rel(&self, address: Option<usize>, value: i32)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    ///Write an i64 to the given address
    fn write_i64_rel(&self, address: Option<usize>, value: i64)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    ///Write an u8 to the given address
    fn write_u8_rel(&self, address: Option<usize>, value: u8)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    ///Write an u32 to the given address
    fn write_u32_rel(&self, address: Option<usize>, value: u32)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    ///Write an u64 to the given address
    fn write_u64_rel(&self, address: Option<usize>, value: u64)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    ///Write an f32 to the given address
    fn write_f32_rel(&self, address: Option<usize>, value: f32)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }

    ///Write an f64 to the given address
    fn write_f64_rel(&self, address: Option<usize>, value: f64)
    {
        let buffer = value.to_ne_bytes();
        self.write_memory_rel(address, &buffer);
    }
}