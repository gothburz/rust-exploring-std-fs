mod dir_builder_mod;

fn main() {
    let path = "./tmp/sub_tmp/sub_sub_tmp";
    dir_builder_mod::dir_builder_mod::create_directories(path, 0o700);
}
