use std::{path::Path, io::Result};

pub fn read_file(file_path: &Path) -> Result<String> {
    std::fs::read_to_string(file_path)
}

pub fn change_CRLF_to_LF(i: String) -> String {
    if cfg!(target_os = "windows") {
        
        i.replace("\r\n", "\n")
    } else {
        print!("CRLF --- ");
        i
    }
}