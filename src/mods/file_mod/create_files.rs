use std::fs;
use std::io::prelude::*;

pub fn create_file_write_all(file_path: &str, content: &[u8]){
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(content).unwrap();
}

pub fn create_file_write(file_path: &str, msg: &[u8]){
    fs::write(file_path, msg).unwrap();
}
