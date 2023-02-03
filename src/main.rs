mod vcr_init;
mod object;

use vcr_init::init;
use object::blob;

use std::io;

// 構造体のインポート
use blob::Blob;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).unwrap().clone();

    let blob = Blob::new(String::from("123456789"));

    println!("{:?}", Blob::as_bytes(&blob));

    match cmd.as_str() {
        "init" => {
            init::create_repository();
            Ok(())
        }
        "test" => {
            println!("引数は{:?}です", cmd);
            Ok(())
        }
        _ => Ok(())
    }
}
