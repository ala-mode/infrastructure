use std::{env, ffi::OsString};
mod fetcher;
fn main() {
    println!(
        "CARGO MANIFEST DIR in build.rs is {:?}",
        env::var("CARGO_MANIFEST_DIR")
    );
    fetcher::main();
}
