use std::fs;
use std::io;

mod cmd;
mod object;

use cmd::{init, add};
use object::{git_object, blob};

use blob::Blob;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).unwrap().clone();

    git_object::test();

    match cmd.as_str() {
        "init" => {
            let result = init::create_repository();

            match result {
                Ok(_) => {
                    println!("Initialized empty Git repository 🎉");
                }
                Err(e) => {
                    println!("{}", e);
                }

            }
        }
        "test" => {
            println!("引数は{:?}です", cmd);
        }
        _ => {
            println!("error!");
        }
    }
}
