use crate::filesystem::FileSystemItem;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    size: usize,
    content: String,
}
impl File {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            name: name,
            size: size,
            content: "".to_string(),
        }
    }
    pub fn add_content(&mut self, content: String) {
        self.content = content;
    }
}
impl FileSystemItem for File {
    fn get_size(&self) -> usize {
        self.size
    }

    fn display(&self) {
        println!("Size: {} File Name:{}\n", self.get_size(), self.name)
    }
}
pub struct Filed;

pub trait A {
    fn info(&self);
}

pub trait B {
    fn info(&self);
}
impl A for Filed {
    fn info(&self) {
        todo!()
    }
}
impl B for Filed {
    fn info(&self) {
        todo!()
    }
}
