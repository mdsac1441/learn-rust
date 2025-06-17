fn main() {
    /*
    Declare a Variable
    Use the let keyword to declare a variable.
    */
    let company_string = "TutorialsPoint";  // string type
    let rating_float = 4.5;                 // float type
    let is_growing_boolean = true;          // boolean type
    let icon_char = '♥';                    //unicode character type
 
    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("company icon is:{}\n",icon_char);
    /* 
    * A scalar type represents a single value. For example, 10,3.14,'c'. Rust has four primary scalar types.
    *Integer
    *Floating-point
    *Booleans
    *Characters
    */
    //Integer
    let result = 10;    // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;
    println!("result value is {}",result);
    println!("sum is {} and age is {}",sum,age);
    println!("mark is {} and count is {}",mark,count);

    // integer overflow

    // let age:u8 = 255;

    // // 0 to 255 only allowed for u8
    // let weight:u8 = 256;   //overflow value is 0
    // let height:u8 = 257;   //overflow value is 1
    // let score:u8 = 258;    //overflow value is 2
 
    // println!("age is {} ",age);
    // println!("weight is {}",weight);
    // println!("height is {}",height);
    // println!("score is {}",score);


    // float

    let results = 10.00;        //f64 by default
    let interest:f32 = 8.35;
    let cost:f64 = 15000.600;  //double precision
    
    println!("results value is {}",results);
    println!("interest is {}",interest);
    println!("cost is {}",cost);
    
    //Number Separator
    let float_with_separator = 11_000.555_001;
    println!("float value {}",float_with_separator);
    
    let int_with_separator = 50_000;
    println!("int value {}",int_with_separator);

    // Boolean

    let isfun:bool = true;
    println!("Is Rust Programming Fun ? {}",isfun);

    // Character

    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';
    
    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("emoji is {}",emoji);


 }
