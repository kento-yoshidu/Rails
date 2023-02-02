mod vcr_init;
use vcr_init::init;

use std::io;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).unwrap().clone();

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
