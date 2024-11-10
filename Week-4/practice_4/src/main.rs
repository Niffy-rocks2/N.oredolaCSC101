use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter your age:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let age: u32 = input.trim().parse().expect("Please type a number!");

    if age >= 18 {
        println!("Welcome to the party, {}!", name.trim());
    } else {
        println!("You are not of age to enter the party, {}!", name.trim());
    }
}

