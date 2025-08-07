    use std::fs;
    
    use::crate::pe::parser;

    pub fn run(exe_path: &str) {
        println!("[loader] Loading: {}", exe_path);
        let bytes = fs::read(exe_path).expect("Failed to read exe file");

        match parer::parse_pe(&bytes) {
            OK(parsed) => {
                println!("[+] PE loaded successfully");
                println!("    Entry point RVA: 0x{:08X}", parsed.entry_point);
                ptintln!("    Image Bass:      0x{:08X}", parsed.image_base);
                println!("    Sections:");
                for (i, sec) in parsed.sections.iter().enumerate() {
                    let name = std::str::from_utf8(&sec.name).unwrap_or("???").trim_end_matches('\0');
                    println!(
                        "   [{}] {:<8} RVA: 0x{:08x}, Size: 0x{:08X}",
                        i,
                        name,
                        sec.virtual_address,
                        sec.virtual_size,
                    );
                }
            }
            Err(e) => {
                eprintln!("[!] Failled to parse PE: {}", e);
            }
        }
    }

