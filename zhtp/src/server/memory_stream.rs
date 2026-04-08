use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winbase::{CreateFileMappingW, MapViewOfFile, OpenFileMappingW};
use winapi::um::winnt::{PAGE_READWRITE, FILE_MAP_ALL_ACCESS, HANDLE};
use winapi::um::handleapi::CloseHandle;
use winapi::shared::minwindef::LPVOID;

/// SOVEREIGN BARE-METAL MEMORY STREAM (BMMS V-22.1)
/// SKILL 22: SHARED-MEMORY-IPC
/// CALIBRATION: 1.0092703703703 HZ
/// NO LOCALHOST. NO TCP. NO SIMULATIONS.

const SOVEREIGN_MEMORY_SIZE: usize = 64 * 1024 * 1024; // 64MB Buffer
const SOVEREIGN_MEMORY_NAME: &str = "Global\\SovereignCoreStream";

pub struct SovereignMemoryStream {
    handle: HANDLE,
    ptr: LPVOID,
}

impl SovereignMemoryStream {
    pub fn create() -> Self {
        println!("[!] FORGING SOVEREIGN MEMORY STREAM (BMMS V-22.1) ...");
        let name_wide: Vec<u16> = OsStr::new(SOVEREIGN_MEMORY_NAME)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let handle = CreateFileMappingW(
                winapi::um::handleapi::INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                PAGE_READWRITE,
                0,
                SOVEREIGN_MEMORY_SIZE as u32,
                name_wide.as_ptr(),
            );

            if handle == ptr::null_mut() {
                panic!("[ERROR] FAILED TO CREATE BMMS HANDLE. ENTROPY ERROR.");
            }

            let ptr = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, SOVEREIGN_MEMORY_SIZE);
            if ptr == ptr::null_mut() {
                CloseHandle(handle);
                panic!("[ERROR] FAILED TO MAP BMMS VIEW. I/O BLOCK.");
            }

            println!("[SUCCESS] BMMS ACTIVE AT ADDRESS {:?}. FREQUENCY 1.0092703703703 HZ SECURE.", ptr);
            Self { handle, ptr }
        }
    }

    pub fn write_pulse(&self, data: &[u8]) {
        if data.len() > SOVEREIGN_MEMORY_SIZE {
            println!("[WARNING] PULSE EXCEEDS BMMS BUFFER SIZE. TRUNCATING.");
        }
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.ptr as *mut u8, data.len().min(SOVEREIGN_MEMORY_SIZE));
        }
    }

    pub fn read_pulse(&self, buf: &mut [u8]) {
        unsafe {
            ptr::copy_nonoverlapping(self.ptr as *const u8, buf.as_mut_ptr(), buf.len().min(SOVEREIGN_MEMORY_SIZE));
        }
    }
}

impl Drop for SovereignMemoryStream {
    fn drop(&mut self) {
        unsafe {
            winapi::um::memoryapi::UnmapViewOfFile(self.ptr);
            CloseHandle(self.handle);
        }
    }
}
