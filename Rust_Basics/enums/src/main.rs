// In Rust programming, 
// when we have to select a value from a list of possible variants we use enumeration data types.
#[derive(Debug)]
enum GenderCategory {
Male,Female
}

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Person {
   name:String,
   gender:GenderCategory
}

enum CarType {
    Hatch,
    Sedan,
    SUV
 }

//  enum Status{
//     Accept,
//     Reject
//  }

// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum Deatil {
   Name(String),Usr_ID(i32)
}

fn main() {
    println!("Hello, Enums");

    // The `derive` attribute automatically creates the implementation
    // required to make this `enum` printable with `fmt::Debug`.
    let male = GenderCategory::Male;
    let female = GenderCategory::Female;

    println!("{:?}",male);
    println!("{:?}\n",female);

    let p1 = Person {
        name:String::from("Mohtashim"),
        gender:GenderCategory::Male
     };
     let p2 = Person {
        name:String::from("Amy"),
        gender:GenderCategory::Female
     };
     println!("{:?}",p1);
     println!("{:?}\n",p2);

    /*
    *Option Enum
    Option is a predefined enum in the Rust standard library. 
    This enum has two values − Some(data) and None.
    */

    let result = is_even(3);
    println!("{:?}",result);
    println!("{:?}",is_even(30));

    // Match Statement and Enum
    // The match statement can be used to compare values stored in an enum. 
    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);

    match is_even(8) {
        Some(data) => {
           if data==true {
              println!("\nEven ");
           }
        },
        None => {
           println!("\nnot even");
        }
     }

    //  let accept=Status::Accept;
    //  println!("\n{:?}",accept);

    let d1 = Deatil::Name(String::from("Md Sharqiue Ahmed"));
    let d2 = Deatil::Usr_ID(100);
    println!("\n{:?}",d1);
    println!("\n{:?}",d2);
 
    match d1 {
       Deatil::Name(val)=> {
          println!("{}",val);
       }
       Deatil::Usr_ID(val)=> {
          println!("{}",val);
       }
    }

}

// fn is_even(no:i32)->Option<String> {
//     if no%2 == 0 {
//        Some("Even".to_string())
//     } else {
//        None
//     }
//  }

fn is_even(no:i32)->Option<bool> {
    if no%2 == 0 {
       Some(true)
    } else {
       None
    }
 }
 

 fn print_size(car:CarType) {
    match car {
       CarType::Hatch => {
          println!("\nSmall sized car");
       },
       CarType::Sedan => {
          println!("\nmedium sized car");
       },
       CarType::SUV =>{
          println!("\nLarge sized Sports Utility car");
       }
    }
 }
