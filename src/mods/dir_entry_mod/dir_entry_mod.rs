use std::fs;
use std::fs::{DirEntry, FileType, Metadata};
use std::path::{Path, PathBuf};

pub fn get_dir_entries(start_dir: &str) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut dir_entries = vec![];
    for dir_entry in fs::read_dir(start_dir)? {
        let dir_entry = dir_entry?;
        dir_entries.push(dir_entry);
    }
    Ok(dir_entries)
}

// pub fn get_dir_entry_metadata(dir_entry: &DirEntry) -> Result<Metadata, E> {
//     for dir_entry in dir_entry_vec {
//        dir_entry.metadata()
//     }
// }

// pub fn get_dir_entry_types(path_vec: &Vec<PathBuf>) -> Result<Vec<FileType>, std::io::Error> {
//     let file_type_vec = vec![];
//     for entry in path_vec {
//         if let Ok(file_type) = entry.file_type() {
//             println!("{:?}: {:?}", entry, file_type);
//         } else {
//             println!("Couldn't get file type for {:?}", entry);
//         }
//     }
//     Ok(file_type_vec)
// }