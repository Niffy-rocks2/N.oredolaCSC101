fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("The city has {} element(s).", city.len());

    // Push new elements into it
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: usize = input1.trim().parse().expect("Invalid input");

    for _ in 0..city_num {
        let mut input2 = String::new();
        println!("Enter city name:");
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        city.push(input2.trim().to_string());
    }

    println!("Your preferred cities are:\n");
    let mut count = 1;

    // Loop to iterate elements in vector
    for i in &city {
        println!("{}: {}", count, i);
        count += 1;
    }
}
