#[derive(Clone)]
pub enum MemoryType
{
    ///Uses the win32 API ReadProcessMemory/WriteProcessMemory functions
    Win32Api,

    ///Assumes this code is running from an injected .dll, uses ptr::read/ptr::write directly
    Direct,
}