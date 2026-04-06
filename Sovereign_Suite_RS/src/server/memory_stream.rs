use std::ptr;
use winapi::um::memoryapi::{CreateFileMappingW, MapViewOfFile, FILE_MAP_ALL_ACCESS};
use winapi::um::winnt::{HANDLE, PAGE_READWRITE};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

pub struct SovereignMemoryStream {
    handle: HANDLE,
    _ptr: *mut u8,
    _size: usize,
}

impl SovereignMemoryStream {
    pub fn create() -> Self {
        let size = 64 * 1024 * 1024;
        let name: Vec<u16> = OsStr::new("SOVEREIGN_CORE_MEMORY_ID")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        unsafe {
            let handle = CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                PAGE_READWRITE,
                0,
                size as u32,
                name.as_ptr(),
            );
            
            let ptr = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, size);
            
            Self {
                handle,
                _ptr: ptr as *mut u8,
                _size: size,
            }
        }
    }
}

impl Drop for SovereignMemoryStream {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

unsafe impl Send for SovereignMemoryStream {}
unsafe impl Sync for SovereignMemoryStream {}
