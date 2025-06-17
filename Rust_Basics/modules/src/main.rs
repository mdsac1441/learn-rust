/*

A logical group of code is called a Module. 
Multiple modules are compiled into a unit called crate. 
Rust programs may contain a binary crate or a library crate. 
A binary crate is an executable project that has a main() method. 
A library crate is a group of components that can be reused in other projects. 
Unlike a binary crate, a library crate does not have an entry point (main() method). 
The Cargo tool is used to manage crates in Rust. 

*/

// crate
// Is a compilation unit in Rust; Crate is compiled to binary or library.

// cargo

// The official Rust package management tool for crates.

// module

// Logically groups code within a crate.

pub mod factorial{

    pub fn fact(num:u32)->u32{
        if num==0{
            1
        }else{
            num*fact(num-1)
        }
    }
}
use std::io;
use factorial::fact;

fn main() {
    println!("Hello, Modules");
    loop{  
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Invalid number");
        let num:u32=number.trim().parse().unwrap();
        println!("{}",fact(num));  
        println!("If You want to Exit, Type y/n :");
        let mut no  = String::new();
        io::stdin().read_line(&mut no).expect("Invalid Strings");
       println!("{}",no);
        if no.trim()=="y" || no.trim()=="Y"{
            println!("in yes");
            break;

        }
    }

}
