#![feature(dir_entry_ext2)]
use std::os::unix::fs::{DirEntryExt2, MetadataExt};

mod mods;

fn main() {
    // let dir_path = "./tmp/";
    // let dir_entries = mods::dir_entry_mod::dir_entries::get_dir_entries(dir_path);
    // match dir_entries {
    //     Ok(dir_entries) => {
    //         for dir_entry in dir_entries {
    //             // FILENAME
    //             let dir_entry_file_name = dir_entry.file_name();
    //             println!("FILE NAME: {:?}", dir_entry_file_name);
    //             println!("FILE NAME REF: {:?}", dir_entry.file_name_ref());
    //             // PATH
    //             let dir_entry_path = dir_entry.path();
    //             println!("PATH: {:?}", dir_entry_path);
    //             // GET METADATA
    //             let dir_entry_metadata = dir_entry.metadata().unwrap();
    //             println!("METADATA: {:?}", dir_entry_metadata);
    //             // GET  PERMISSIONS
    //             let dir_entry_permissions = dir_entry_metadata.permissions();
    //             println!("dir_entry_permissions: {:?}", dir_entry_permissions);
    //             // GET IS_DIR BOOLEAN
    //             let dir_entry_is_dir_bool = dir_entry_metadata.is_dir();
    //             println!("dir_entry_is_dir_bool: {:?}", dir_entry_is_dir_bool);
    //             // GET IS_SYMLINK BOOL
    //             let dir_entry_is_symlink_bool = dir_entry_metadata.is_symlink();
    //             println!("dir_entry_is_symlink_bool: {:?}", dir_entry_is_symlink_bool);
    //             // GET IS_FILE BOOL
    //             let dir_entry_is_file_bool = dir_entry_metadata.is_file();
    //             println!("dir_entry_is_file_bool: {:?}", dir_entry_is_file_bool);
    //             // GET METADATA FILETYPE
    //             let dir_entry_meta_filetype = dir_entry_metadata.file_type();
    //             println!("dir_entry_meta_filetype: {:?}", dir_entry_meta_filetype);
    //             // GET Unix MODE
    //             let dir_entry_mode = dir_entry_metadata.mode();
    //             println!("dir_entry_mode: {:?}", dir_entry_mode);
    //             // GET Byte LEN of file
    //             let dir_entry_byte__len = dir_entry_metadata.len();
    //             println!("dir_entry_len: {:?}", dir_entry_byte_len);
    //             // GET ACCESSED TIME
    //             let dir_entry_accessed = dir_entry_metadata.accessed();
    //             println!("dir_entry_accessed: {:?}", dir_entry_accessed);
    //             // GET CREATED TIME
    //             let dir_entry_created = dir_entry_metadata.created();
    //             println!("dir_entry_created: {:?}", dir_entry_created);
    //             // GET MODIFIED TIME
    //             let dir_entry_modified = dir_entry_metadata.modified();
    //             println!("dir_entry_modified: {:?}", dir_entry_modified);
    //
    //
    //             // GET FILETYPE
    //             let dir_entry_filetype = dir_entry.file_type().unwrap();
    //             println!("FILE TYPE: {:?}", dir_entry_filetype);
    //         }
    //     }
    //     // Error Handling
    //     _ => {}
    // }
    let msg_write_all = b"Hello from write_all()!";
    let msg_write = b"Hello from write()!";
    let file_to_read = "./example.txt";
    let file_to_read_other = "./example-other.txt";
    mods::file_mod::create_files::create_file_write_all(file_to_read, msg_write_all);
    mods::file_mod::create_files::create_file_write(file_to_read_other, msg_write);
    let file_string_contents = mods::file_mod::reading_files::read_file_string(file_to_read).unwrap();
    println!("read_file_string(): {:?}", file_string_contents);
    let file_vec_contents = mods::file_mod::reading_files::read_file_vec("./example.txt").unwrap();
    println!("read_file_string_vec(): {:?}", file_vec_contents);
    let bufreader_str = mods::file_mod::reading_files::read_with_bufreader_str(file_to_read).unwrap();
    println!("read_with_bufreader_str(): {:?}", bufreader_str);
    let bufreader_vec = mods::file_mod::reading_files::read_with_bufreader_vec(file_to_read).unwrap();
    println!("read_with_bufreader_vec(): {:?}", bufreader_vec);

}
