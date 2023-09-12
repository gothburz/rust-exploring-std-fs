use std::fs;
use std::os::unix::fs::DirBuilderExt;

fn create_directories(path_str: &str, permission_u32: u32) {
    // Create a new DirBuilder object
    let mut builder = fs::DirBuilder::new();
    // Set the builder to create directories recursively
    builder.recursive(true);
    // Set the builder to assign UNIX permissions assigned by the function caller
    builder.mode(permission_u32);
    match builder.create(path_str) {
        Ok(()) => println!("The path {:?} was created!", path_str),
        Err(e) => println!("ERROR: {:?}", e)
    }
}

fn main() {
    let path = "./tmp/sub_tmp/sub_sub_tmp";
    create_directories(path, 0o700);
}
