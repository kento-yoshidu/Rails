use std::fs;


pub fn vcr_init() -> Result<(), String> {
    fs::create_dir("./.vcr").expect("Error")
}
