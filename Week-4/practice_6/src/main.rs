use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let upper_bound: u32 = input.trim().parse().expect("Please type a number!");

    for number in 1..=upper_bound {
        println!("{}", number);
    }
}

