/*

Generics are a facility to write code for multiple contexts with different types. 
In Rust, generics refer to the parameterization of data types and traits. 
Generics allows to write more concise and clean code by reducing code duplication and providing type-safety. 
The concept of Generics can be applied to methods, functions, structures, enumerations, collections and traits.
                                       OR
* When we want to create the function of multiple forms, i.e., 
the parameters of the function can accept the multiple types of data. 
This can be achieved through generics. 
Generics are also known as 'parametric polymorphism' where poly is multiple, 
and morph is form.

* There are two ways to provide the generic code:
1. Option<T>
2. Result<T, E>

*/

struct Data<T> {
    value:T,
 }
fn main() {
    println!("Hello, Generics and Traits");

    // The <T> syntax known as the type parameter, is used to declare a generic construct. 
    // T represents any data-type.

    let mut vector_integer: Vec<i32> = vec![20,30];
    vector_integer.push(40);
    println!("{:?}\n",vector_integer);


    let t1:Data<u32>=Data { value: 25 };
    let t2:Data<String>=Data { value: "Ahmed Chistey".to_string() };
    println!("value is :{} ",t1.value);
    println!("value is :{} ",t2.value);

    // 1. Option<T>: Rust standard library provides Option where 'T' is the generic data type. It provides the generic over one type.

    //  enum Option<T>  
    // {  
    //     Some(T),  
    //     None,  
    // }  

    // In the above case, enum is the custom type where <T> is the generic data type. 
    // We can substitute the 'T' with any data type. Let's look at this:

    
    // let x : Option<i32> = Some(10);  // 'T' is of type i32.  
    // let x : Option<bool> = Some(true);  // 'T' is of type bool.  
    // let x : Option<f64> = Some(10.5); // 'T' is of type f64.  
    // let x : Option<char> = Some('b'); // 'T' is of type char.   

    // In the above case, we observe that 'T' can be of any type, i.e., i32, bool, f64 or char. 
    // But, if the type on the left-hand side and the value on the right hand side didn't match, then the error occurs.

    // 2. Result<T,E>: Rust standard library provides another data type Result<T,E> which is generic over two type, i.e., T &E:

    //     enum Result<T,E>  
    //     {  
    //        OK(T),  
    //          Err(E),  
    //  }   


    // fn function_name<T>(x:T)   

    // The above syntax has two parts:

    // <T> : The given function is a generic over one type.
    // (x : T) : x is of type T.

    // fn function_name<T,U>(x:T, y:U)  

    // let a = vec![1,2,3,4,5];  
    // let b = vec![2.3,3.3,4.3,5.3];  
    // let result = add(&a);  
    // let result1 = add(&b);  
    // println!("The value of result is {}",result);  
    // println!("The value of result1 is {}",result1);  




    // Traits : A trait in Rust is a group of methods that are defined for a particular type.

    //create an instance of the structure
    let b1 = Book {
        id:1001,
        name:"Rust in Action"
     };
     b1.print();

}

// Need to resolve this method
// fn add<T>(list:&[T])->T  
// {  
//   let mut c =0;  
//   for &item in list.iter()  
//   {  
//     c= c+item;  
//   }  
//   c
// }  


//declare a structure
struct Book {
    name:&'static str,
    id:u32
 }
 //declare a trait
 trait Printable {
    fn print(&self);
 }
 //implement the trait
 impl Printable for Book {
    fn print(&self){
       println!("Printing book with id:{} and name {}",self.id,self.name)
    }
 }