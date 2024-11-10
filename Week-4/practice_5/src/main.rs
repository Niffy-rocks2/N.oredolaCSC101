use std::io;

fn main() {
    println!("Enter your height in cm:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let height: u32 = input.trim().parse().expect("Please type a number!");

    if height >= 180 {
        println!("You are tall");
    } else if height >= 170 {
        println!("You are average height");
    } else {
        println!("You are short");
    }
}
