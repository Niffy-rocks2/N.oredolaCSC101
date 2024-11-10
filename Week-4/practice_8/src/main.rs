fn main() {
    let mut number = 1;

    while true {  // Infinite loop
        println!("Inside loop, number value is {}", number);
        number = number + 1;

        if number == 5 {
            break;  // Exit the loop when number reaches 5
        }
    }

    println!("Outside loop, number value is {}", number);  // Print final value after loop
}


