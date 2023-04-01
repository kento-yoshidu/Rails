use sha1::{Digest, Sha1};

#[derive(Debug)]
pub struct Blob {
    pub size: usize,
    pub content: String
}

impl Blob {
    pub fn new(content: String) -> Self {
        Self {
            size: content.len(),
            content
        }
    }

    pub fn calc_hash(&self) -> Vec<u8> {
        Vec::from(Sha1::digest(&self.as_bytes()).as_slice())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("{} {}\0", ObjectType::Blob.to_string(), self.size);
        let store = format!("{}{}", header, self.to_string());

        Vec::from(store.as_bytes())
    }
}
