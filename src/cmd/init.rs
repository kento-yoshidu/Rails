use std::fs;
use std::io::Write;

pub fn create_repository() {
    fs::create_dir("./.vcrust").expect("Reinitialized existing VCrust repository ❌");

    fs::create_dir("./.vcrust/refs").expect("Failed to create directory: .vcrust/refs ❌");

    fs::File::create("./.vcrust/refs/main").expect("Failed to create file: .vcrust/refs/main ❌");

    fs::File::create("./.vcrust/index").expect("Failed to create file: .vcrust/index ❌");

    let mut head = fs::File::create("./.vcrust/HEAD").expect("Failed to create file: .vcrust/HEAD ❌");

    head.write_all(b"ref: refs/heads/main").unwrap();

    println!("Initialized empty Git repository 🎉");
}
