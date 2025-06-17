/*
Rust allocates everything on the stack by default. 
You can store things on the heap by wrapping them in smart pointers like Box. 
Types like Vec and String implicitly help heap allocation.

1. Deref--> std::ops::Deref::Used for immutable dereferencing operations, like *v.
2. Drop--> std::ops::Drop::Used to run some code when a value goes out of scope. This is sometimes called a destructor
*/

use std::ops::Deref;
fn main() {
    println!("Hello, Smart Pointer");

    /*
    1. Box
    The Box smart pointer also called a box allows you to store data on the heap rather than the stack. 
    The stack contains the pointer to the heap data. 
    A Box does not have performance overhead, other than storing their data on the heap.
    */

    let var_i32 = 5; 
    //stack
    let b = Box::new(var_i32); 
    //heap
    println!("b = {}", b);

    // In order to access a value pointed by a variable, use dereferencing. 
    // The * is used as a dereference operator.

    let x = 5; 
    //value type variable
    let y = Box::new(x); 
    //y points to a new value 5 in the heap
 
    println!("{}",5==x);
    println!("{}",5==*y); 
    //dereferencing y

    let num = 5;
    let c = MyBox::new(num); 
    // calling static method
    
    println!("5==x is {}",5==num);
    println!("5==*y is {}",5==*c); 
    // dereferencing y
    println!("x==*y is {}",num==*c);
    //dereferencing y


    let z = 50;
    MyBox::new(z);
    MyBox::new("Hello");



}

struct MyBox<T>(T);
impl<T> MyBox<T> { 
   // Generic structure with static method new
   fn new(x:T)-> MyBox<T> {
      MyBox(x)
   }
}
impl<T> Deref for MyBox<T> {
   type Target = T;
   fn deref(&self) -> &T {
      &self.0 //returns data
   }
}

impl<T> Drop for MyBox<T>{
    fn drop(&mut self){
       println!("dropping MyBox object from memory ");
    }
 }