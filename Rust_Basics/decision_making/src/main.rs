use std::io;
fn main() {
    // let num:i32 = 6;
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num:u32=num.trim().parse().expect("failed to parse the string to number");
    if num > 0 {
       println!("number is positive") ;
    }

    if num < 5{
        println!("YO");
    }else{
        println!("Bye")
    }


    if num > 5{
        println!("more than 5");
    }else if num < 5{
        println!("less than 5");
    }else{
        println!("Equal");
    }
}
