use std::fs;

pub fn create_repository() {
    fs::create_dir("./.vcrust").expect("Reinitialized existing VCrust repository");

    println!("Initialized empty Git repository");
}
