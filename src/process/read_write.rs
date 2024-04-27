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