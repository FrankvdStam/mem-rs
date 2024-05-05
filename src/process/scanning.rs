use crate::helpers::{scan, to_pattern};
use crate::pointer::Pointer;
use crate::prelude::*;

impl Process
{
    pub fn scan_abs(&self, error_name: &str, pattern: &str, scan_offset: usize, pointer_offsets: Vec<usize>) -> Result<Pointer, String>
    {
        let byte_pattern = to_pattern(pattern);
        let scan_result = scan(&self.process_data.borrow().main_module.memory, &byte_pattern);
        if scan_result.is_none()
        {
            return Err(String::from(format!("Scan failed: {}", error_name)));
        }

        let mut address = scan_result.unwrap();
        address += self.process_data.borrow().main_module.base_address;
        address += scan_offset;
        return Ok(Pointer::new(self.process_data.clone(), true, address, pointer_offsets));
    }

    pub fn scan_rel(&self, error_name: &str, pattern: &str, scan_offset: usize, instruction_size: usize, pointer_offsets: Vec<usize>) -> Result<Pointer, String>
    {
        let byte_pattern = to_pattern(pattern);
        let scan_result = scan(&self.process_data.borrow().main_module.memory, &byte_pattern);
        if scan_result.is_none()
        {
            return Err(String::from(format!("Scan failed: {}", error_name)));
        }

        let address = scan_result.unwrap();
        let address_value = self.read_u32_rel(Some(address + scan_offset));
        let result = self.process_data.borrow().main_module.base_address + address + instruction_size + address_value as usize; //Relative jump

        return Ok(Pointer::new(self.process_data.clone(), true, result, pointer_offsets));
    }

    pub fn create_pointer(&self, address: usize, pointer_offsets: Vec<usize>) -> Pointer
    {
        return Pointer::new(self.process_data.clone(), true, address, pointer_offsets);
    }
}