// Declare and initialize a mutable array
let mut colors = ["red", "green", "yellow", "white"];

// Print the original array
println!("Original array: {:?}", colors);

// Create a mutable slice
let mut sliced_colors = &mut colors[1..3];

// Change the value of the first element in the slice
sliced_colors[0] = "purple";

// Print the changed slice
println!("Changed slice: {:?}", sliced_colors);

// Print the original array to see the changes
println!("Original array after slice modification: {:?}", colors);