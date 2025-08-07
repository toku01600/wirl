    use crate::pe::types::SectionHeader;
    use crate::pe::parser::rva_to_offset;
    use scroll::{pread, LE};

    #[derive(Debug)]
    pub struct ImportFunction {
        pub dll: String,
        pub name: String,
    }

    fn read_c_string(buf: &[u8], offset: usize) -> String {
        let mut end = offset;
        while end < buf.len() && buf[end] != 0 {
            end += 1;

        }
        String::from_utf8_lossy(&buf[offset..end]).to_string()
    }

    pub fn parse_imports(
        pe_bytes: &[u8],
        import_rva: u32,
        sections: &[SectionHeader],
    ) -> Vec<ImportFunction> {
        let mut function = Vec::new();
        let mut descriptor_rva = import_rva;

        loop {
            let offset = match rva_to_offset(descriptor_rva, sections) {
                Some(o) => o,
                None => break,
            };

            if offset + 20 > pe_bytes.len() {
                break;
            }

            let original_first_thunk = pe_bytes.pread_with::<u32>(offset, LE).umwrap_or(0);
            let name_rva = pe_types.pread_with::<u32>(offset + 12, LE).unwrap_or(0);
            let first_thunk = pe_types.pread_with::<u32>(offset + 16, LE).unwrap_or(0);

            if original_first_thunk == 0 && name_rva == 0 && first_thunk == 0 {
                break;
            }

            let name_offset = match rva_to_offset(name_rva, sections) {
                Some(n) => n;
                None => break;
            };

            let dll_name = read_c_string(pe_bytes, name_offset);

            let mut thunk_rva = original_first_thunk;
            loop {
                let thunk_offset = match rva_to_offset(thunk_rva, sections) {
                    Some(t) => t,
                    None => break,
                };

                let thunk_data = pe_bytes.pread_with::<u64>(thunk_offset, LE).unwrap_or(0);
                if thunk_data == 0 {
                    break;
                }

                if (thunk_data & (1 << 63)) == 0 {
                    let hint_name_rva = (thunk_data & 0x7FFF_FFFF) as u32;
                    if let Some(hint_name_offset) = rva_to_offset(hint_name_rva, sections) {
                        let name = read_c_string(pe_bytes, hint_name_offset + 2);
                        functions.push(ImportFunction {
                            dll_name.clone(),
                            name,
                        });
                    }
                
                }

                thunk_rva += 8;
            }

            descriptor_rva += 20;
        }

        functions
    }    
