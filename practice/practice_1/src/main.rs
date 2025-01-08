use std::io;
 fn main() {
     let mut name = String::new();
     println!("What is your name?");

   // input
   io::stdin()
     .read_line(&mut name)
     .expect("Failed to read input"); 

println!("your name is {}",name);

//AGE
 let mut age = String::new();
 println!("how old are thy?");

 //input
 io::stdin()
 .read_line(&mut age)
 .expect("Failed to read input");
 let age:i32 = age.trim().parse().expect("input not an integer");
 println!("Your age is {}",age);
}
