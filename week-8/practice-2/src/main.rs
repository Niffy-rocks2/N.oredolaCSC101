fn main() {
    let v = vec!['C', 'O', 'D', 'E', 'R'];

    let mut input = String::new();

    println!("Enter an index value btw (0 - 4):");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    // Index is the non-negative value which is smaller than the size of the vector
    let index: usize = input.trim().parse().expect("Invalid input");

    // Getting value at given index value
    let ch: char = v[index];

    println!("{} is the character for index [{}]", ch, index);
}
