use std::fs;

fn main() {
    fs::remove_file("Welcome_message.txt").expect("could not remove file");
    println!("File is removed");
}
