// Cargo is the package manager for RUST. 
// This acts like a tool and manages Rust projects.

/*

1. cargo build--> Compiles the current project.

2. cargo check--> Analyzes the current project and report errors, but don't build object files.

3. cargo run--> Builds and executes src/main.rs.

4. cargo clean--> Removes the target directory.

5. cargo update--> Updates dependencies listed in Cargo.lock.

6. cargo new--> Creates a new cargo project.

Create a binary crate--> cargo new project_name --bin
Create a library crate--> cargo new project_name --lib

*/

use rand::{thread_rng,Rng};
use std::cmp::Ordering;
use std::io;



fn main() {
    println!("Hello, Package Manager\n");

    println!("Welcome GUESS gaming :");

    let win_num=thread_rng().gen_range::<u8,_>(1..101);
    loop{

        println!("Enter You Guessing Number: \n");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess:u8= guess.trim().parse().unwrap();
        println!(" Your Guess Number is {}",guess);
        

    match guess.cmp(&win_num){
            Ordering::Less=>println!("Your guessing number is too low"),
            Ordering::Greater=>println!("Your guessing number is too high "),
            Ordering::Equal=>{
                println!("Congo!,You Win the Game ");
                break;
            } 
        }

    }
}
