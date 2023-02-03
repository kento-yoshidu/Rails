use crate::blob::Blob;

#[derive(Debug)]
pub enum GitObject {
    Blob(Blob),
}

impl GitObject {
    pub fn new() {
        // オブジェクトタイプによっていずれかを生成する
    }
}

pub fn test() {
    let b = Blob::new(String::from("hogehgoe"));

    println!("{:?}", b);
}
