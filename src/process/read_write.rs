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

use crate::prelude::{BaseReadWrite, Process, ReadWrite};

impl BaseReadWrite for Process
{
    fn read_memory_rel(&self, offset: Option<usize>, buffer: &mut [u8]) -> bool
    {
        let mut address = self.process_data.borrow().main_module.base_address;
        if offset.is_some()
        {
            address += offset.unwrap();
        }
        return self.read_with_handle(self.process_data.borrow().handle, address, buffer);
    }

    fn write_memory_rel(&self, offset: Option<usize>, buffer: &[u8]) -> bool
    {
        let mut address = self.process_data.borrow().main_module.base_address;
        if offset.is_some()
        {
            address += offset.unwrap();
        }
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

impl ReadWrite for Process{}