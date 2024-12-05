use std::fs::File;
use std::io::Read;

pub fn read_txt(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Couldn't open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Couldn't read file");
    file_content
}
