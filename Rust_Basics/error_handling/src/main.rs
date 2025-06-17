/*
* In Rust, errors can be classified into two major categories
1. Recoverable: Errors which can be handled-->Result enum.
2. UnRecoverable: Errors which cannot be handled-->panic macro.

*/
use std::fs::File;
fn main() {
    println!("Hello, Error Handling");

    // Panic Macro and Unrecoverable Errors
    // panic! macro allows a program to terminate immediately and provide feedback to the caller of the program.
    // It should be used when a program reaches an unrecoverable state.

    // panic!("Hello");
    println!("End of main"); //unreachable statement

    // let a = [10,20,30];
    // a[10]; //invokes a panic since index 10 cannot be reached

    let no = 10; 
    //try with odd and even
    if no%2 == 0 {
       println!("Thank you , number is even");
    } else {
       panic!("NOT_AN_EVEN"); 
    }
    println!("End of main\n");

    // Result Enum and Recoverable Errors
    // Enum Result – <T,E> can be used to handle recoverable errors. 
    // It has two variants − OK and Err. T and E are generic type parameters. 
    // T represents the type of the value that will be returned in a success case within the OK variant, 
    // and E represents the type of the error that will be returned in a failure case within the Err variant.

    // enum Result<T,E> {
    //     OK(T),
    //     Err(E)
    // }

    let f=File::open("file.pdf");
    // println!("{:?}",f);

    match f {
        Ok(f)=> {
           println!("file found {:?}",f);
        },
        Err(e)=> {
           println!("file not found \n{:?}",e);   //handled error
        }
    }
    println!("end of main");

    let result = is_even(13);
     match result {
        Ok(d)=>{
           println!("no is even {}",d);
        },
        Err(msg)=>{
           println!("Error msg is {}",msg);
        }
     }
    println!("end of main\n");

    // unwrap() and expect():
    //The standard library contains a couple of helper methods that both enums 
    //− Result<T,E> and Option<T> implement. 

    let result1 = is_even(10).unwrap();
    println!("result is {}",result1);
    println!("end of main\n");

    let f1 = File::open("pqr.txt").expect("File not able to open");
    //file does not exist
    println!("end of main");

}

fn is_even(no:i32)->Result<bool,String> {
    if no%2==0 {
       return Ok(true);
    } else {
       return Err("NOT_AN_EVEN".to_string());
    }
 }
