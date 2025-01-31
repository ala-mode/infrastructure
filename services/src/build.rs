use std::{env, ffi::OsString};
mod fetcher;
pub fn main() {
    let manifest_dir = std::path::PathBuf::from(
        //&(env::var("CARGO_MANIFEST_DIR").expect("cargo manifest dir to unwrap")),
        env::current_dir().expect("env current dir to unwrap"),
    );
    let path_thingy = manifest_dir
        .join("fetched_resourcezzz")
        .join("test_binaries")
        .join("zuz.txt");
    //let prefix = path.parent().unwrap();
    std::fs::create_dir_all(path_thingy).expect("path thingy to unwrap");
    fetcher::main();
}
