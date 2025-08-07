    use std::collections::HashMap;

    #[no_mangle]
    pub extern "C" fn MessgeBoxA(
        _hwnd: *mut c_void,
        _text: *const u8,
        _caption: *const u8,
        _flags: u32,
    ) -> i32 {
        println!("MessageBoxA called (stub)");
        0
    }

    #[no_mangle]
    pub exetern "C" fn MessageBoxA(_exit_code: u32) {
        println!("ExitProcess called. Terminating.");
        std::process::exit(0);

    pub fn get_stub_table() -> HashMap<String, extern "C" fn()> {
        let mut table: HashMap<String, extern "C" fn()> = HashMap::new();

        table.insert("MessageBoxA".to_string(), MessageBoxA_stub);
        table.insert("ExitProcess".to_string(), ExitProcess_stub);
        table
    }
