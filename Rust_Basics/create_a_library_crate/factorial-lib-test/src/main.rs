extern crate factorial_lib;
use factorial_lib::factorial::fact;
use std::io;
fn main() {
    println!("inside main of test ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Invalid number");
    let num:u32=number.trim().parse().unwrap();
    println!("{}",fact(num)); 
}