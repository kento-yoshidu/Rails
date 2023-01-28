use std::fs;
use std::path::PathBuf;

fn vcr_init() {
    // let srcdir = PathBuf::from("./vcrust");

    fs::create_dir("./.vcrust").expect("Reinitialized existing VCRust repository");
}

fn main() {
    vcr_init();
}
