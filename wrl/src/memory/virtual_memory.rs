    use std::collections::HashMap;
    use std::ptr::NonNull;

    use crate::memory::exec_mem::ExecutableMemory;

    pub struct VirtualMemory {
        mapped: HashMap<u32, Vec<u8>>,

        exec_mapped: HashMap<u32, ExecutableMemory>,
    }
    
    impl VirtualMemory {
        pub fn new() -> Self {
            Self {
                mapped: HashMap::new(),
                exec_mapped: HashMap::new(),
        }
    }

    pub fn map_section(&mut self, va: u32, data: Vec<u8>, executable: bool {
        if executable {
            let exec_mem = ExecutableMemory::new(data);
            self.exec_mapped.insert(va, exec_mem);
        } else {
            self.mapped.insert(va, data);
        }
    }

    pub fn read(&self, va: u32, size: usize) -> Option<&[u8]> {
        for (base, region) in &self.mapped {
            let len = region.len() as u32;
            if va >= *base && va + size as u32 <= *base + len  {
                let offset = (va - base) as usize;
                        return Some(&region[offset..offset + size]);
                    }
                }
                
                for (base, exec_mem) in &self.exec_mapped {
                    let len = exec_mem.len() as u32;
                    if va >= *base && va + size as u32 <= *base + len {
                        let offset = (va - base) as usize;
                        return Some(&exec_mem.as_slice()[offset..offset + size]);
                        
                    }
                
            }
        
        None
    }

    pub fn write(&mut self, va: u32, data: &[u8]) -> bool {
        for (base, region) in self.mapped.iter_mut() {
            let len = region.len() as u32;
            if va >= *base && va + data.len() as u32 <= *base + len {
                let offset = (va - base) as usize;
                region[offset..offset + data.len()].copy_from_slice(data);
                return true;
            }   
            
        }
        false
    }
    pub fn get_host_address(&self, va: u32) -> Option<*const u8> {
        for (base, exec_mem) in &self.exec_mapped {
            let len = exec_mem.len() as u32;
            if va >= *base && va < *base + len {
                let offset = (va - base) as usize;
                return Some(unsafe { exec_mem.ptr().as_ptr().add(offset) });
            }
        }
        None
    }

