/* 
*The memory for a program can be allocated in the following −
Stack
Heap
 */

fn main() {
    println!("Hello, Ownership");

    /* 
    *Transferring Ownership
    The ownership of value can be transferred by −

    *Assigning value of one variable to another variable.

    *Passing value to a function.

    *Returning value from a function.
    */


    // Assigning value of one variable to another variable.
    let v = vec![1,2,3]; 
    // vector v owns the object in heap
 
    //only a single variable owns the heap memory at any given time
    let v2 = v; 
    // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust
 
    //Rust is very smart in terms of memory access ,so it detects a race condition
    //as two variables point to same heap
 
    // It means the ownership is moved from v to v2 (v2=v) and v is invalidated after the move.
    // println!("{:?}",v);

    // Passing value to a function
    // display1(v2);             // v2 is moved to display and v2 is invalidated
    // println!("In main {:?}",v3);    //v2 is No longer usable here
    let v3=display2(v2);
    println!("In main {:?}",v3);   

    /*
    *Ownership and Primitive Types
    In case of primitive types, contents from one variable is copied to another. 
    So, there is no ownership move happening. 
    This is because a primitive variable needs less resources than an object.
    */

    let u1=11;
    let u2=u1; // u1 value copied(not moved) to u2
    println!("u1 = {}",u1);


}

// fn display1(v:Vec<i32>){
//     println!("inside display {:?}",v);
// }

fn display2(v:Vec<i32>)->Vec<i32>{
    // returning same vector
    // println!("inside display {:?}",v);
    v
}
