use std::io;

fn main() {
    println!("Student Information Management System");

    // Input name
    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is: {}", name.trim());

    // Input age
    println!("Enter your age:");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");
    let age: i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);
}
