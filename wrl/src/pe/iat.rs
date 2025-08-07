    use crate::memory::virtual_memory::VirtualMemory;
    use crate::pe::import::ImportFunction;
    use std::collections::HashMap;

    pub fn construct_iat(
        imports: &[ImportFunction],
        vm: &mut VirtualMemory,
        iat_base: u32,
        stub_table: &HashMap<String, extern "C" fn()>,
    ) {
        let mut offset = 0;


        for Import in imports {
            if let Some(&func_ptr) = stub_table.get(&import.name) {
                let fn_ptr = host_fn as *const () as usize as u32;
                let addr = iat_base + offset;

                let bytes = fn_ptr.to_le_bytes().to_vec();
                vm.map_section(addr, bytes.clone(), false);

                println!(
                    "[IAT] {}!{}  → 0x{:08X} Already written"
                    func.dll, finc.name, fn_ptr
                );

                offset += 4;
            } else {
                println!(
                    "[warn] Stub not defined: {}!{} →  skip", 
                    func.dll, func.name
                );
            }
        }
    }
