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
}

fn main() {
    println!("Hello, world!");

    let a = Blob::new(String::from("Hello World"));

    println!("{:?}", a);
}
