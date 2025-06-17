/*

*The println!("{ }",tuple) syntax cannot be used to display values in a tuple.
This is because a tuple is a compound type. 
Use the println!("{:?}", tuple_name) syntax to print values in a tuple.
*Destructing assignment is a feature of rust wherein we unpack the values of a tuple. 
This is achieved by assigning a tuple to distinct variables
*/

// fn print(num:(i32,u32,f32)){
//     println!("{:?}",num);
// }

fn print(num:(u32,bool,f32)){
    let (roll_num,is_pass,percentage)=num;
    println!("roll_num is : {}\nis_pass : {}\npercentage : {}",roll_num,is_pass,percentage);
}

fn main() {
    println!("Hello, Tuple");
    let create_tuple=(1,2,3,"Md Sharique Ahmed");
    let create_another_tuple:(i32,f32,String)=(5,5.00,"Ahmed".to_string());
    // let number:(i32,u32,f32)=(-11,42,3.14);
    let student:(u32,bool,f32)=(2,true,80.5);
    println!("{:?}",create_tuple);
    println!("{:?}",create_another_tuple);
    // print(number);
    print(student)


}
