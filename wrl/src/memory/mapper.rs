    use crate::pe::types::SectionHeader;
    use crate::memory::virtual_memory::VirtualMemory;

    
    pub fn map_pe_to_memory(pe_bytes: &[u8], sections: &[crate::pe::types::SectionHeader], vm: &mut VirtualMemory) {
        for section in sections {
            let start = section.pointer_to_raw_data as usize;
            let end = start + section.size_of_raw_data as usize;

            if end > pe_bytes.len() {
                eprintln!("Warning: section {} is out of bounds", section.name);
                continue;
            }

            let section_data = pe_bytes[start..end].to_vec();

            let is_executable = section.name.trim_end_matches('\0') == ".text";

            vm.map_section(section.virtual_address, section_data, is_execcutable);
        }
    }
