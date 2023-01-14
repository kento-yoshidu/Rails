use std::fmt;
use sha1::{Digest, Sha1};

pub enum GitObject {
    Blog(Blob),
    // Tree(Tree),
    // Commit(Commit)
}

#[derive(Debug)]
pub struct Blob {
    pub size: usize,
    pub content: String
}

impl Blob {
    // contentからBlobを生成
    pub fn new(content: String) -> Self {
        Self {
            size: content.len(),
            content
        }
    }

    // byte列からBlobを生成
    pub fn from(bytes: &[u8]) -> Option<Self> {
        let content = String::from_utf8(bytes.to_vec());

        match content {
            Ok(content) => Some(Self {
                size: content.len(),
                content
            }),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("blob {}\0", self.size);
        let store = format!("{}{}", header, self.to_string());

        Vec::from(store.as_bytes())
    }

    pub fn calc_hash(&self) -> Vec<u8> {
        Vec::from(Sha1::digest(&self.as_bytes()).as_slice())
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

pub struct Tree {
    pub contents: Vec<File>
}

pub struct File {
    pub mode: usize,
    pub name: String,
    pub hash: Vec<u8>
}

impl File {
    pub fn new(mode: usize, name: String, hash: &[u8]) -> Self {
        Self {
            mode,
            name,
            hash: hash.to_vec()
        }
    }

    pub fn from(header: &[u8], hash: &[u8]) -> Option<Self> {
        let split_header = String::from_utf8(header.to_vec()).ok()?;

        let mut iter = split_header.split_whitespace();

        let mode = iter.next().and_then(|x| x.parse::<usize>().ok())?;
        let name = iter.next()?;

        Some(Self::new(mode, String::from(name), hash))
    }   
}

fn main() {
    println!("Hello, world!");

    let a = Blob::new(String::from("Hello World"));
    println!("{:?}", Blob::calc_hash(&a));
}
