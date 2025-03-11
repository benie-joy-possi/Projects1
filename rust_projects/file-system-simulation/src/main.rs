use file::{A, B, File, Filed};
use filesystem::FileSystemItem;
use folder::Folder;

fn main() {
    println!("Hello, world!");
    let file1 = File::new("data.txt".to_string(), 100);
    let file2 = File::new("data.txt".to_string(), 100);
    if file1 == file2 {
        println!("Files are identical")
    }
    // let f = Filed;
    // A::info(&f);
    // B::info(&f);
    let mut file_1 = Box::new(File::new("/usb".to_string(), 4));
    file_1.add_content("this is the content of file 1".to_string());

    let mut file_2 = Box::new(File::new("/flash".to_string(), 4));
    file_2.add_content("this is the content of file 2".to_string());

    let mut file_3 = Box::new(File::new("/key".to_string(), 4));
    file_3.add_content("this is the content of file 3".to_string());

    let mut file_4 = Box::new(File::new("/key".to_string(), 4));
    file_4.add_content("this is the content of file 3".to_string());
    // println!("{:#?}", file_1.eq(&file_2));
    // file_1.display();

    let folder_5 = Folder::new("/exo");
    let mut folder_1 = Folder::new("/dev");
    folder_1.add_item(file_1);
    folder_1.add_item(file_2);
    folder_1.add_item(Box::new(folder_5));
    let mut folder_2 = Folder::new("/name");
    let mut folder_3 = Folder::new("/another name");
    folder_3.add_item(file_3);
    folder_2.add_item(file_4);
    folder_2.add_item(Box::new(folder_3));

    folder_1.add_item(Box::new(folder_2));

    let mut root = Folder::new("root");
    root.add_item(Box::new(folder_1));

    // // let r =  root.iter()
    // for item in root.iter() {
    //     // println!("{:#?}", item);
    //     item.display();
    // }
    root.display();
}
mod file;
mod filesystem;
mod folder;
