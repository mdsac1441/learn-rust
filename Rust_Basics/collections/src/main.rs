use std::collections::{HashMap,HashSet};
fn main() {
    // vec
    println!("Hello, Collections");
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
 
    println!("\nsize of vector is :{}",v.len());
    println!("{:?}",v);
    v.remove(1);
    println!("{:?}",v);

    let v1 = vec![1,2,3];
    println!("\n{:?}",v1);

    if v1.contains(&1) {
        println!("\nfound 1");
     }else{
        print!("\nnot found 1");
     }
     println!("\n{:?}",v1);

     println!("size of vector is :{}",v1.len());

     // Accessing values from a Vector
     println!("{:?}",v1[0]);

     let mut v2=Vec::new();
     v2.push(20);
     v2.push(10);
     v2.push(30);
     v2.push(40);

     for i in &v2 {
        println!("{}",i);
     }
     println!("{:?}",v2);

    // HashMap 

    // A map is a collection of key-value pairs (called entries). 
    // No two entries in a map can have the same key. In short, a map is a lookup table. 
    // A HashMap stores the keys and values in a hash table

     let mut h1=HashMap::new();
     h1.insert("name", "Sharique");
     h1.insert("age", "23");
     println!("{:?}",h1);
     println!("size of map is {}",h1.len());

     match h1.get(&"name") {
        Some(value)=> {
           println!("Value for key name is {}",value);
        }
        None => {
           println!("nothing found");
        }
     }


    for (key, val) in h1.iter() {
        println!("key: {}-> val: {}", key, val);
     }


    if h1.contains_key(&"age") {
        println!("found key");
     }else{
        println!("not found key");
     }

     h1.remove(&"age");
     println!(" HashMap Length after remove operation : {}\n",h1.len());

    // HashSet
    //HashSet is a set of unique values of type T. 
    //Adding and removing values is fast, and it is fast to ask whether a given value is in the set or not.
    // //duplicates not added.
    let mut h2 =HashSet::new();
    h2.insert("Md Sharique Ahmed");
    h2.insert("Danish Faraz");
    h2.insert("CPN Singh");
    h2.insert("Chirag");
    println!("{:?}",h2);
    println!("size of the set is {}\n",h2.len());
    for name in h2.iter(){
        println!("{}\n",name);
    }

    match h2.get(&"Chirag"){
        Some(name)=>println!("found :{}\n",name),
        None=> println!(" not found \n")
    }

    if h2.contains(&"Kannan") {
        println!("found name");
     }else{
        println!("not found name");
     }

}
