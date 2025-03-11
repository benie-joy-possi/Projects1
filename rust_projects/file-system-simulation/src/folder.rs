use crate::filesystem::FileSystemItem;
#[derive(Debug)]
pub struct Folder {
    name: String,
    items: Vec<Box<dyn FileSystemItem>>,
}
impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }
    pub fn add_item(&mut self, item: Box<dyn FileSystemItem>) {
        self.items.push(item);
    }
}

impl FileSystemItem for Folder {
    fn get_size(&self) -> usize {
        let mut size: usize = 0;
        for item in &self.items {
            size += item.get_size()
        }
        size
    }

    fn display(&self) {
        println!("Folder:{}", self.name);
        for item in &self.items {
            item.display();
        }
    }
}

struct FolderIterator<'a> {
    folder: &'a Folder,
    index: usize,
}

impl<'a> Iterator for FolderIterator<'a> {
    type Item = &'a Box<dyn FileSystemItem>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.folder.items.len() {
            return None;
        }
        self.index += 1;
        Some(&self.folder.items[self.index - 1])
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        println!(" {:?} has just been deleted ", self.name)
    }
}

pub fn display_tree(folder: &Folder, depth: usize) {
    for item in (FolderIterator { folder, index: 0 }) {
        item.display();
    }
}
