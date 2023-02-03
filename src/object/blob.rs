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

    pub fn as_bytes(&self) {
        println!("self is {:?}", self.size);
    }
}
