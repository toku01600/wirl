    mod loader;
    mod memory;
    mod pe;

    use std::env;
    use std::fs;
    use std::mem::transmute;
    use crate::memory::virtual_memory::VirtualMemory;
    use crate::memory::mapper::map_pe_to_memory;
    use crate::pe::parser::parse_pe;
    use crate::pe::imports::parse_imports;
    use crate::pe::iat::construct_iat;
    use crate::pe::api_stubs::{MessageBoxA, ExitProcess};
    use crate::pe::api_stubs::get_stubs_table;
    use memory::exec_mem::ExecutableMemory;

    fn main() {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: wrl <program.exe>");
            std::process::exit(1);
        }

        let exe_path = &args[1];

        let pe_bytes = fs::read(exe_path).expect("Failed to load PE file");

        let parsed = parse_pe(&pe_bytes).expect("PE parsing failure");
        let section_headers = &parsed.sections;

        let mut vm = VirtualMemory::new();
        
        map_pe_to_memory(&pe_bytes, &section_headers, &mut vm);

        for section in section_headers {
            println!(
                "Mapped section: {} at VA 0x{:08X}, size: {}",
                section.name,
                section.virtual_address,
                section.virtual_size,
            );
        }

        pub mod api_stubs;
        
        let imports = parse_imports(&pe_bytes, parsed.import_directory_rva, &section_headers);
        let stub_table = get_stub_table();

        let iat_base = 0x3000;
        construct_iat(&imports, &mut vm, iat_base, &stub_table);
        println!("IAT constructed at VA: 0x{:08X}", iat_base);
        
        vm.map_section(0x1000, vec![1, 2, 3, 4, 5], false);
        if let Some(data) = vm.read(0x1002, 2) {
            println!("read: {:?}", data);
        }

        let success = vm.write(0x1001, &[0xAA, 0xBB]);
        println!("Write successful: {}", success);

        let entry_va = parsed.entry_point;
        println!("Jumping to enyty point: 0x{:08X}", entry_va);

        let host_address = vm.get_host_address(entry_va).unwrap();

        let entry_funk: extern "C" fn() = unsafe {
            std::mem::transmute(host_address) 
        };
        
        unsafe {
            entry_func();
        }
    }

