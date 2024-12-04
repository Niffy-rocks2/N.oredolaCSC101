fn main() {
    let num: i32 = 56;
    mutate_num_to_zero(num);
    println!("The value of num is: {}", num); // `num` is still 56
}

fn mutate_num_to_zero(mut param_num: i32) {
    param_num = 0;
    println!("param_num value is: {}", param_num); // Prints 0
}


