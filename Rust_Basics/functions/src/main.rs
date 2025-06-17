fn factorial(num:u32)->u32{
    if num==0 {
        return 1;
    }else{
       return num*factorial(num-1);
    }
}


// fn mutate_no_to_zero(mut param_no: i32) {
//     param_no = param_no*0;
//     println!("param_no value is :{}",param_no);
//  }


fn mutate_no_to_zero(param_no:&mut i32) {
    *param_no = 0;
    println!("param_no value is :{}",param_no);
 }

// use std::io;

fn main() {

    // if cfg!(target_os = "linux") {
    //     println!("Yes. It's definitely linux!");
    // } else {
    //     println!("Yes. It's definitely *not* linux!");
    // }

    // let mut number=String::new();
    // io::stdin().read_line(&mut number).unwrap();
    // let num:u32=number.trim().parse().unwrap();
    // println!("{}",factorial(num));

    // let no:i32 = 5;
    // mutate_no_to_zero(no);
    // println!("The value of no is:{}",no);

    let mut no:i32 = 5;
    mutate_no_to_zero(&mut no);
    println!("The value of no is:{}",no);

}
