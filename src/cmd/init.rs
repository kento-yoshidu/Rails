use std::io;
use std::fs;
use std::io::Write;

pub fn create_repository() -> Result<(), String> {
    match fs::create_dir("./.vcrust") {
        Ok(_) => {}
        Err(_) => {
            return Err("❌ Reinitialized existing VCRust repository.".to_string());
        }
    }

    match fs::create_dir("./.vcrust/refs") {
        Ok(_) => {}
        Err(_) => {
            fs::remove_dir_all(".vcrust");
            return Err("❌ Failed to create directory: .vcrust/refs.".to_string());
        }
    }
    
    match fs::File::create("./.vcrust/refs/main") {
        Ok(_) => {}
        Err(_) => {
            fs::remove_dir_all(".vcrust");
            return Err("❌ Failed to create file: .vcrust/refs/main ❌".to_string());
        }
    }

    match fs::File::create("./.vcrust/index") {
        Ok(_) => {}
        Err(_) => {
            fs::remove_dir_all(".vcrust");
            return Err("❌ Failed to create file: .vcrust/index".to_string());
        }
    }

    let mut head = fs::File::create("./.vcrust/HEAD").expect("Failed to create file: .vcrust/HEAD ❌");

    head.write_all(b"ref: refs/heads/main").unwrap();

    return Ok(());
}
