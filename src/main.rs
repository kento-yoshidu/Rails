use std::env;
use sha1::{Sha1, Digest};
use std::path::PathBuf;

/*
use std::fs;
use std::io;

mod cmd;
mod object;

use cmd::{init, add};
use object::{git_object, blob};

use blob::Blob;
*/

fn main() {
    let blob = format!("blob {}\0{}", 11, "Hello World");

    println!("{}", blob);

    let blob_object = Sha1::digest(blob.as_bytes());

    println!("{:x}", blob_object);
    //=> 5e1c309dae7f45e0f39b1bf3ac3cd9db12e7d689

    let mut path = PathBuf::new();

    path.push(env::current_dir().expect("絶対パスの取得"));

    println!("{}", path);

    path.push("text.txt");

    println!("{}", path.display());

    /*
    let abs_path = env::current_dir()?;

    let file = PathBuf::from("text.txt");

    println!("{}", file.display());

    Ok(())
    */

    /*
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).unwrap().clone();

    // let blob = Blob::new("hogehoge".to_string());

    // println!("{:?}", blob);

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
    */
}
