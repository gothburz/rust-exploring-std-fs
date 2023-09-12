use std::fs;

pub fn get_dir_entries(read_dir_path: &str) -> Result<Vec<fs::DirEntry>, std::io::Error> {
    let mut dir_entries = vec![];
    for dir_entry in fs::read_dir(read_dir_path)? {
        let dir_entry = dir_entry?;
        dir_entries.push(dir_entry);
    }
    Ok(dir_entries)
}