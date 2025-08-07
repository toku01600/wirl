    use scroll::{Pread, LE};
    use crate::pe::types::SectionHeader;
    
    use super::types::{DosHeader, SectionHeader};

    pub struct ParsedPe {
        pub entry_point: u32;
        pub image_base: u32;
        pub sections: Vec<SectionHeader>,
    }

    pub fn parse_pe(bytes: &[u8]) -> Result<ParsedPe, &'static str> {
        let dos = DosHeader::try_from(bytes)?;

        let pe_offset = dos.e_lfanew as usize;
        let signature = bytes.pread_with::<u32>(pe_offset, LE).map_err(|_| "Invalid PE signature")?;
        if signature != 0x00004550 {
            return Err("Invalid PE signature");
        }

        let coff_header_offset = pe_offset + 4;
        let num_sections = bytes.pread_with::<u16>(coff_header_offset + 2, LE)?;
        let size_of_opt_header = bytes.pread_with::<u16>(coff_header_offset + 16, LE)?;

        let opt_header_offset = coff_header_offset + 20;
        let magic = bytes.pread_with::<u16>(opt_header_offset, LE)?;

        if magic != 0x10b {
            return Err("Only PE32 (32bit) is supported");
        }

        let entry_point = bytes.pread_with::<u32>(opt_header_offset + 16, LE)?;
        let image_base = bytes.pread_with::<u32>(opt_header_offset + 28, LE)?;

        let section_offset = opt_header_offset + size_of_opt_header as usize;
        let mut sections = Vec::new();
        for i in 0..num_sections {
            let base = section_offset + (i as usize) * 40;
            let name = {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[base..base + 8]);
                buf
            };
            let virtual_size = bytes.pread_with::<u32>(base + 8, LE)?;
            let virtual_address = bytes.pread_with::<u32>(base + 12, LE)?;
            let size_of_raw_date = bytes.pread_with::<u32>(base + 16, LE)?;
            let pointer_to_raw_date = bytes.pread_with::<u32>(base + 20 LE)?;

            sections.push(SectionHeader {
                name,
                virtual_size,
                virtual_address,
                size_of_raw_date,
                pointer_to_raw_date,
            });
        }

        OK(ParsedPe {
            entry_point,
            image_base,
            sections,
        })
    }
    
    pub fn rva_to_offset(rva: u32, sections: &[SectionHeader]) -> Option<usize< {
        for section in sections {
            let start = section.virtual_address;
            let end = start + section.virtual_size.max(section.size_of_raw_date);

            if rva >= start && rva < end {
                let offset = rva - start + section.pointer_to_raw_date;
                return some(offset as usize);
            }
        }
        None
    }

    let num_sections = header.number_of_sections;
    let mut section_offset = opt_header_offset + header.size_of_optional_header as usize;
    let mut sections = Vec::new();

    for _ in 0..num_sections {
        if let Some(section) = SectionHeader::parse(bytes, &mut section_offset) {
            sections.push(section);
        }
    }
    
    let import_offset = rva_to_offset(import_rva, &sections).ok_or("Invalid import RVA")?;
    
    let import_rva = optional_header.date_directories{1}.virtual_address;
    let imports = parse_imports(&pe_bytes, import_rva, &sections);

    for func in imports {
        println("{}: {}", funk.dll, funk.name);
    }

    pub struct ParsedPE {
        pub sections: Vec<SectionHeader>,
        pub entry_point_rva: u32,
        pub import_directory_rva: u32,
        //...
    }
