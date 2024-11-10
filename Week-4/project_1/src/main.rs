use std::io;

fn main() {
    println!("Enter the coefficients a, b, and c of the quadratic equation:");

    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    io::stdin()
        .read_line(&mut a_input)
        .expect("Failed to read a");
    io::stdin()
        .read_line(&mut b_input)
        .expect("Failed to read b");
    io::stdin()
        .read_line(&mut c_input)
        .expect("Failed to read c");

    let a: f64 = a_input.trim().parse().expect("Please type a number!");
    let b: f64 = b_input.trim().parse().expect("Please type a number!");
    let c: f64 = c_input.trim().parse().expect("Please type a number!");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The roots are real and equal: {}", root);
    } else {
        println!("The roots are imaginary.");
    }
}