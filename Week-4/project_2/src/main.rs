use std::io;

fn main() {
    // Input for experience and age
    println!("Is the employee experienced? (yes/no): ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experienced = experience.trim().to_lowercase() == "yes";

    println!("Enter the age of the employee: ");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    // Determine the incentive based on the criteria
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age < 40 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0
        }
    } else {
        100_000
    };

    println!("The annual incentive for the employee is: ₦{}", incentive);
}
