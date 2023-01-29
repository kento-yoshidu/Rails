use std::fs;
use std::fs::File;

pub fn create_repository() {
    fs::create_dir("./.vcrust").expect("Reinitialized existing VCrust repository");

    fs::create_dir("./.vcrust/refs").expect("Failed to create directory: .vcrust/refs");

    File::create("./.vcrust/refs/main").expect("Failed to create file: .vcrust/refs/main");

    File::create("./.vcrust/index").expect("Failed to create file: .vcrust/index");

    println!("Initialized empty Git repository");
}
