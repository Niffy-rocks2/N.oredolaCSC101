// Method to print the get value
fn value(ch: Option<&char>) {
    println!("Element of vector: {:?}", ch);
}

fn main() {
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'A', 'D', 'E', 'M', 'Y'];

    let mut input = String::new();
    println!("Enter an index value btw (0 - 10):");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    // Index is the non-negative value which is smaller than the size of the vector
    let index: usize = input.trim().parse().expect("Invalid input");

    // Getting value at given index value
    let ch: Option<&char> = v.get(index);

    value(ch);
}
