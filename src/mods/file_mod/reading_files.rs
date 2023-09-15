use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// READ ENTIRE FILE INTO A STRING
pub fn read_file_string(read_path: &str) -> Result<String,  std::io::Error> {
    let mut file = File::open(read_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    Ok(file_contents)
}


// READ ENTIRE FILE INTO A BYTES VECTOR
pub fn read_file_vec(read_path: &str) -> Result<Vec<u8>,  std::io::Error>{
    let mut file = File::open(read_path)?;
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();
    Ok(data)
}

// READ USING BufReader INTO STRING
pub fn read_with_bufreader_str(read_path: &str) -> Result<String,  std::io::Error> {
    let mut file = File::open(read_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut file_contents = String::new();
    buf_reader.read_to_string(&mut file_contents).unwrap();
    Ok(file_contents)
}

// READ USING BufReader INTO VECTOR
pub fn read_with_bufreader_vec(read_path: &str) -> Result<Vec<u8>,  std::io::Error> {
    let mut file = File::open(read_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut data = vec![];
    buf_reader.read_to_end(&mut data).unwrap();
    Ok(data)
}