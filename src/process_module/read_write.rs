use crate::memory::{BaseReadWrite, ReadWrite};
use crate::prelude::ProcessModule;

impl BaseReadWrite for ProcessModule
{
    fn read_memory_rel(&self, offset: Option<usize>, buffer: &mut [u8]) -> bool
    {
        let mut address = self.base_address;
        if offset.is_some()
        {
            address = address + offset.unwrap(); //unsure if this is intuitive
        }
        return self.read_with_handle(self.process_data.borrow().handle, self.process_data.borrow().memory_type.clone(), address, buffer);
    }

    fn write_memory_rel(&self, offset: Option<usize>, buffer: &[u8]) -> bool
    {
        let mut address = self.base_address;
        if offset.is_some()
        {
            address = address + offset.unwrap(); //unsure if this is intuitive
        }
        return self.write_with_handle(self.process_data.borrow().handle, self.process_data.borrow().memory_type.clone(), address, buffer);
    }

    fn read_memory_abs(&self, address: usize, buffer: &mut [u8]) -> bool
    {
        return self.read_with_handle(self.process_data.borrow().handle, self.process_data.borrow().memory_type.clone(), address, buffer);
    }

    fn write_memory_abs(&self, address: usize, buffer: &[u8]) -> bool
    {
        return self.write_with_handle(self.process_data.borrow().handle, self.process_data.borrow().memory_type.clone(), address, buffer);
    }
}

impl ReadWrite for ProcessModule{}