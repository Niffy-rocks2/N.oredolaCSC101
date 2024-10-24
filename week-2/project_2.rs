fn main() {
    // Array of numbers
    let numbers = [450,000, 1,500,000, 750,000, 2,850,000, 250,000];

    // Calculate the sum
    let sum: i32 = numbers.iter().sum();

    // Calculate the average
    let count = numbers.len();
    let average = sum as f64 / count as f64;

    // Print the results
    println!("Sum: {}", sum);
    println!("Average: {:.2}", average); // Display with 2 decimal places
}

