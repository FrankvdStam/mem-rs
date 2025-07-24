use std::ffi::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

pub trait BaseReadWrite
{
    /// Read memory relative to the object's location in memory. Supports an optional offset.
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
    /// let mut buffer: [u8; 8] = [0; 8];
    /// let success = pointer.read_memory_rel(Some(0x1234), &mut buffer);
    /// ```
    fn read_memory_rel(&self, offset: Option<usize>, buffer: &mut [u8]) -> bool;

    /// Write memory relative to the object's location in memory. Supports an optional offset.
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
    /// let mut buffer: [u8; 4] = [0x1, 0x2, 0x3, 0x4];
    /// let success = pointer.write_memory_rel(Some(0x1234), &mut buffer);
    /// ```
    fn write_memory_rel(&self, offset: Option<usize>, buffer: &[u8]) -> bool;

    /// Read memory from an absolute address
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
    /// let mut buffer: [u8; 8] = [0; 8];
    /// let success = pointer.read_memory_abs(0x1234, &mut buffer);
    /// ```
    fn read_memory_abs(&self, address: usize, buffer: &mut [u8]) -> bool;

    /// Write memory to an absolute address
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
    /// let mut buffer: [u8; 4] = [0x1, 0x2, 0x3, 0x4];
    /// let success = pointer.write_memory_abs(0x1234, &mut buffer);
    /// ```
    fn write_memory_abs(&self, address: usize, buffer: &[u8]) -> bool;

    /// Read memory into a buffer from a process handle
    fn read_with_handle(&self, handle: HANDLE, address: usize, buffer: &mut [u8]) -> bool
    {
        let mut read_bytes = 0;
        if unsafe {ReadProcessMemory(handle, address as *mut c_void, buffer.as_mut_ptr() as *mut c_void, buffer.len(), Some(&mut read_bytes)).is_err() }
        {
            return false;
        }
        return read_bytes == buffer.len();
    }

    /// Write from a buffer ino memory from a process handle
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