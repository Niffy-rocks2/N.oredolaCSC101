fn main(){
let p:f64 = 210000.0;
let r:f64 = 5.0;
let t:f64 = 3.0;

 // A
 let a = p * (1.0-(r / 100.0 )) * t;
 println!("Amount is {}", a);
 let ci = a - p;
 println!("compound interest is {}", ci);
}