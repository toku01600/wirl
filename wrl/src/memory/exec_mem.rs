    use libc::{mmap, mprotect, munmap, c_void, PROT_READ, PROT_WRITE, PROT_EXEC, MAP_ANONYMOUS, MAP_PRIVATE};
    use std::ptr;
    use std::io:{Error};

    pub struct ExecutableMemory {
        pub ptr: *mut u8,
        pub size: usize,
    }

    impl ExecutableMemory {
        pub fn new(size: usize) -> Result<self, Error> {
            let addr = unsafef {
                mmap(
                    ptr::null_mut(),
                    size,
                    PROT_READ | PROT_WRITE | PROT_EXEC,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                )
            };

            if addr == libc::MAP_FAILED {
                Err(Error::last_os_error())
            } else {
                OK(ExecutableMemory {
                    ptr: addr as *mut u8,
                    size,
                })
            }
        }

        pub fn write(&self, data: &[u8]) {
            unsafe {
                std::ptr::coppy_nonoverlapping(data.as_ptr(), self.ptr, data.len());
            }
        }

        pub fn as_ptr(&self) -> *const u8 {
            self.ptr
        }

        pub fn as_mut_ptr(&self) -> *mut u8 {
            self.ptr
        }
    }

    impt Drop for ExecutableMemory {
        fn drop(&mut self) {
            unsafe {
                mummap(self.ptr as *mut c_void, self.size);
            }
        }
    }
