#![feature(dir_entry_ext2)]
use std::fs::DirEntry;
use std::io::Error;
use std::os::unix::fs::DirEntryExt2;

mod mods;

fn main() {
    let path = "./tmp/sub_tmp/sub_sub_tmp";
    mods::dir_builder_mod::dir_builder_mod::create_directories(path, 0o700);

    let dir_path = "./tmp/";
    let dir_entries = mods::dir_entry_mod::dir_entry_mod::get_dir_entries(dir_path);
    match dir_entries {
        Ok(dir_entries) => {
            for dir_entry in dir_entries {
                // FILENAME
                let dir_entry_file_name = dir_entry.file_name();
                println!("FILE NAME: {:?}", dir_entry_file_name);
                println!("FILE NAME REF: {:?}", dir_entry.file_name_ref());
                // PATH
                let dir_entry_path = dir_entry.path();
                println!("PATH: {:?}", dir_entry_path);
                // GET METADATA
                let dir_entry_metadata = dir_entry.metadata().unwrap();
                println!("METADATA: {:?}", dir_entry_metadata);
                // GET FILETYPE
                let dir_entry_filetype = dir_entry.file_type().unwrap();
                println!("FILE TYPE: {:?}", dir_entry_filetype);
            }
        }
        _ => {}
    }
    // let dir_entries = dir_entries?;
    // for dir_entry in dir_entries {
    //    println!(dir_entry.metadata())
    // }
}
