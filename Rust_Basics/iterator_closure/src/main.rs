fn main() {
    println!("Hello, Iterator and Closure");

    //declare an array
    let a = [10,20,30];

    let mut iter = a.iter(); 
    // fetch an iterator object for the array
    println!("{:?}",iter);

    //fetch individual values from the iterator object
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());

    let names1 = vec!["Kannan", "Mohtashim", "Kiran"];
    for name in names1.iter() {
       match name {
          &"Mohtashim" => println!("There is a rustacean among us!"),
          _ => println!("Hello {}\n", name),
       }
    }
    println!("{:?}",names1); 
    // reusing the collection after iteration

    // into_iter()

    let names2 = vec!["Kannan", "Mohtashim", "Kiran"];
    for name in names2.into_iter() {
       match name {
          "Mohtashim" => println!("There is a rustacean among us!"),
          _ => println!("Hello {}", name),
       }
    }
    // cannot reuse the collection after iteration
    // println!("{:?}",names2); 
    //Error:Cannot access after ownership move

    // iter_mut()
    let mut names3 = vec!["Kannan", "Mohtashim", "Kiran"];
    for name in names3.iter_mut() {
       match name {
          &mut "Mohtashim" => {
            println!("There is a rustacean among us!")},
          _ => println!("Hello {}", name),
       }
    }
    println!("{:?}",names3);
    //// reusing the collection after iteration
    
    // Closure refers to a function within another function. 
    // These are anonymous functions – functions without a name. 
    // Closure can be used to assign a function to a variable. 
    // This allows a program to pass a function as a parameter to other functions. 
    // Closure is also known as an inline function. 
    // Variables in the outer function can be accessed by inline functions.

    // let closure_function = |parameter| {
    //     //logic
    // }

    let is_even = |x| {
        x%2==0
     };
     let no = 13;
     println!("{} is even ? {}",no,is_even(no));

     let val = 10; 
     // declared outside
     let closure2 = |x| {
        x + val //inner function accessing outer fn variable
     };
     println!("{}",closure2(2));

}
