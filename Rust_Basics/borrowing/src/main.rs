/*

It is very inconvenient to pass the ownership of a variable to another function and then return the ownership. 
Rust supports a concept, borrowing, 
where the ownership of a value is transferred temporarily to an entity and then returned to the original owner entity.

*/

fn main() {
    println!("Hello, Borrowing");

    // a list of nos
   let v = vec![10,20,30];
//    print_vector(v);
//    println!("{}",v[0]); // this line gives error

/*

*What is Borrowing?
When a function transfers its control over a variable/value to another function temporarily, 
for a while, it is called borrowing. 
This is achieved by passing a reference to the variable (& var_name) 
rather than passing the variable/value itself to the function. 

*/

print_vector(&v); // passing reference
println!("Printing the value from main() v[0]={}",v[0]);

let mut i = 3;
add_one(&mut i);
println!("add_one {}", i);

let mut name:String = String::from("Md Sharique");
display(&mut name); 
//pass a mutable reference of name
println!("The value of name after modification is:{}",name);

}

fn print_vector(x:&Vec<i32>){
    println!("Inside print_vector function {:?}",x);
    
}

fn add_one(e: &mut i32) {
    *e+= 1;
 }

 fn display(param_name:&mut String){
    println!("param_name value is :{}",param_name);
    param_name.push_str(" Ahmed");
    //Modify the actual string,name
 }
