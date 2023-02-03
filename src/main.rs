use std::io;

mod cmd;
mod object;

use cmd::{init, add};
use object::{git_object, blob};

use blob::Blob;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).unwrap().clone();

    git_object::test();

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
