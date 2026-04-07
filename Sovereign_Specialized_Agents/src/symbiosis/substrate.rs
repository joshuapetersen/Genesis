use std::sync::Arc;

/// SUBSTRATE ADAPTER (V-132.8)
/// Goal: Provide a platform-agnostic interface for the Sovereign Hive's shared memory substrate.

pub trait SubstrateAdapter: Send + Sync {
    /// Connect to or create the shared memory substrate.
    fn connect(name: &str, size: usize) -> Arc<Self> where Self: Sized;
    
    /// Retrieve the raw pointer to the mapped memory.
    fn get_ptr(&self) -> *mut u8;
    
    /// Synchronize and finalize the substrate connection.
    fn sync(&self);
}

#[cfg(windows)]
pub mod windows {
    use super::SubstrateAdapter;
    use std::sync::Arc;
    use windows::Win32::System::Memory::{
        CreateFileMappingW, MapViewOfFile, FILE_MAP_ALL_ACCESS, PAGE_READWRITE,
    };
    use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE, CloseHandle};
    use windows::core::{PCWSTR, HSTRING};

    pub struct WindowsSubstrate {
        handle: HANDLE,
        ptr: *mut u8,
    }

    impl SubstrateAdapter for WindowsSubstrate {
        fn connect(name: &str, size: usize) -> Arc<Self> {
            let name_hstring = HSTRING::from(name);
            let name_pcwstr = PCWSTR(name_hstring.as_ptr());
            
            unsafe {
                let handle = CreateFileMappingW(
                    INVALID_HANDLE_VALUE,
                    None,
                    PAGE_READWRITE,
                    0,
                    size as u32,
                    name_pcwstr,
                ).expect("Failed to create Win32 memory mapping");
                
                let ptr_raw = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, size);
                if ptr_raw.Value.is_null() {
                    panic!("Failed to map Win32 view");
                }
                
                Arc::new(Self {
                    handle,
                    ptr: ptr_raw.Value as *mut u8,
                })
            }
        }

        fn get_ptr(&self) -> *mut u8 {
            self.ptr
        }

        fn sync(&self) {
            // Win32 MapViewOfFile is coherent for local mapping.
        }
    }

    impl Drop for WindowsSubstrate {
        fn drop(&mut self) {
            unsafe { let _ = CloseHandle(self.handle); }
        }
    }

    unsafe impl Send for WindowsSubstrate {}
    unsafe impl Sync for WindowsSubstrate {}
}
