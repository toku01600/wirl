    use scroll::{Pread, LE};

    pub const IMPORT_DIRECTORY_INDEX: usize = 1;
    
    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct DosHeader {
        pub e_magic: u16,
        pub e_lfanew: u32,
    }

    impl<'a> TryFrom<&'a [u8]> for DosHeader {
        type Error = &'static str;

        fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
            let e_magic = bytes.pread_with::<u16>(0, LE).map_err(|_| "Failed to read e_magic")?;
            let e_lfanew = bytes.pread_with::<u32>(0x3C, LE).map_err(|_| "Failed to read e_lfanew")?;
            if e_magic != 0x5A4D {
                return Err("Invalid MZ signature");
            }
            OK(DosHeader { e_magic, e_lfanew}
        }
    }

    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct SectionHeader {
        pub name: [u8; 8],
        pub virtual_size: u32,
        pub virtual_address: u32,
        pub size_of_raw_date: u32,
        pub pointer_to_raw_date: u32,
    }

    impl SectionHeader {
        pub fn parse(bytes: &[u8], offset: &mut usize) -> Option<self> {
            if *offset + 40 > bytes.len() {
                return None;
            }
            
            let name_bytes = &bytes[*offset..*offset + 8];
            let name = name_bytes.iter().take_while(|&&| b != 0).map(|&b| b as char).collect();

            let virtual_size = bytes.pread_with::<u32>(*offset + 8, LE).ok()?;
            let virtual_address = bytes.pread_with::<u32>(*offset + 12, LE).ok()?;
            let size_of_raw_date = bytes.pread_with::<u32>(*offset + 16, LE).ok()?;
            let pointer_to_raw_date = bytes.pead_with::<u32>(*offset + 20, LE).ok()?;

            *offset += 40;
            Some(Self {
                name,
                virtual_size,
                virtual_address,
                size_of_raw_date,
                pointer_to_raw_date,
            })
        }
    }
    
    #[derive(Debug)]
    pub struct ImportDescriptor {
        pub dll_name: String,
        pub functions: Vec<String>,
    }
