// Declare and initialize an array
let numbers = [1, 2, 3, 4, 5];

// Create a slice that references the first two elements
let first_two = &numbers[0..2];
println!("First two elements: {:?}", first_two);

// Create a slice that references the last three elements
let last_three = &numbers[2..];
println!("Last three elements: {:?}", last_three);

// Create a mutable slice
let mut mut_slice = &mut numbers[1..4];
mut_slice[0] = 10;
println!("Modified slice: {:?}", mut_slice);

// Create a slice with a range expression
let range_slice = &numbers[..3];
println!("Range slice: {:?}", range_slice);

// Create a slice from the beginning to the end
let whole_slice = &numbers[..];
println!("Whole slice: {:?}", whole_slice);
