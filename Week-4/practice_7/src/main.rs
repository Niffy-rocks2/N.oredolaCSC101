fn main() {
    let mut number = 1;

    while number <= 10 {
        println!("Inside loop, number value is {}", number);
        number = number + 1;
    }

    println!("Outside loop, number value is {}", number);
}
