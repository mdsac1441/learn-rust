use serde::{Serialize, Deserialize};
use serde_json::{Result,Value};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person{
    name:&'static str,
    roll_number:u32,
    age:u32,
    address:&'static str,
    total_marks:f32,

}
trait TPerson{
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name:&'static str,roll_number:u32,age:u32,address:&'static str, total_marks:f32) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    // fn roll_number(&self) -> N;
    // // Traits can provide default method definitions.
    // fn total_marks(&self,m1:N,m2:N,m3:N,m4:N,m5:N)->f64;
}

// Need to resolve
impl TPerson for Person{
    fn new(name:&'static str,roll_number:u32,age:u32,address:&'static str,total_marks:f32) -> Person {  
        Person { name: name, roll_number: roll_number, age: age, address: address,total_marks:total_marks}
    }
    fn name(&self)->&'static str{
        self.name
    }
}

fn main() {
    let point:Point = Point { x: 1, y: 2 };

    println!("Orginal{:?}",point);
    // Convert the Point to a JSON string.
    let serialized:String = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Value = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    let tperson:Person=TPerson::new("Ahmed", 1, 24, "address", 0.0);
    println!("Orginal{:?}",tperson);
    // Convert the Point to a JSON string.
    let serialized1:String = serde_json::to_string(&tperson).unwrap();
    println!("serialized={}",serialized1);
    // Convert the JSON string back to a Point.
    let deserialized1: Value = serde_json::from_str(&serialized1).unwrap();
    println!("deserialized = {:?}", deserialized1);

}
