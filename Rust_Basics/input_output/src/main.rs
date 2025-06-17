/*

This chapter discusses how to accept values from the standard input (keyboard) 
and display values to the standard output (console). 
In this chapter, we will also discuss passing command line arguments.

*/

// Rust’s standard library features for input and output are organized around two traits −
// 1.Read->stdin,File --> byte-oriented 
// 2.Write->stdout,File -->Write support both byte-oriented and UTF-8 text output. 
use std::io::Write;
use std::io;
use std::env::args;
fn main() {
    println!("Hello, Input and Output");

    // let mut line = String::new();
    // println!("Enter your name :");
    // let b1 = io::stdin().read_line(&mut line).unwrap();
    // println!("Hello , {}", line);
    // println!("no of bytes read , {}\n", b1);

    let c1 = io::stdout().write("Tutorials ".as_bytes()).unwrap();
    let c2 = io::stdout().write(String::from("Point").as_bytes()).unwrap();
    io::stdout().write(format!("\nbytes written {}",(c1+c2)).as_bytes()).unwrap();

    /* 
    CommandLine arguments are passed to a program before executing it. 
    They are like parameters passed to functions. 
    CommandLine parameters can be used to pass values to the main() function. 
    */

    let cmd_line = args();
    println!("\nNo of elements in arguments is :{}",cmd_line.len()); 
    //print total number of values passed
    // for arg in cmd_line {
    //    println!("[{}]",arg); //print all values passed as commandline arguments
    // }

    let mut sum = 0;
    let mut has_read_first_arg = false;
 
    //iterate through all the arguments and calculate their sum
 
    for arg in cmd_line {
       if has_read_first_arg { //skip the first argument since it is the exe file name
          sum += arg.parse::<i32>().unwrap();
       }
       has_read_first_arg = true; 
       // set the flag to true to calculate sum for the subsequent arguments.
    }
    println!("sum is {}",sum);

}
