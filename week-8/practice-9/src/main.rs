fn main() {
    // Declare and initialize a tuple
    let my_tuple = (10, 20, 30);

    // Pass the tuple as an argument to a function
    print_tuple(my_tuple);
}

fn print_tuple(t: (i32, i32, i32)) {
    // Access and print the elements of the tuple
    println!("The values are: {}, {}, {}", t.0, t.1, t.2);
}
