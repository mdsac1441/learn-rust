// Array element values can be updated or modified but cannot be deleted.
// Arrays are static. This means that an array once initialized cannot be resized.
fn main() {
    println!("Hello, Array");
    let arr1=[1,2,45,2];
    println!("{:?}",arr1);
    println!("Array length : {}",arr1.len());
    let arr2:[u32;5]=[1,2,3,8,6];
    println!("{:?}",arr2);

    // The following example creates an array and initializes all its elements with a default value of -1.

    let arr3:[i32;5]=[-1;5];
    println!("{:?}",arr3);
    println!("Array size is :{}",arr3.len());

    println!("Array with for looP START");
    for i in 0..=3{
        println!("index is: {} & value is : {}",i,arr1[i]);
    }

    println!("Array with for loop iter() START");
    // The iter() function fetches values of all elements in an array.
    for i in arr1.iter(){
        println!("{}",i);
    }

    let mut arr4:[i32;4] = [10,20,30,40];
    arr4[1] = 0;
    println!("Mutable Array {:?}",arr4);
    println!("Array with function parameters :");
    // Pass by value
    let arr5:[u32;3]=[1,4,9];
    update_array1(arr5);
    println!("Inside main(Pass by value) {:?}",arr5);

    // Pass by referance
    let mut arr6:[u32;4]=[1,2,8,6];
    update_array2(&mut arr6);
    println!("Inside main(Pass by referance){:?}",arr6);

    // let N: usize = 20;
    // let arr = [0; N]; //Error: non-constant used with constant
    // print!("{}",arr[10])
    /* 
    The compiler will result in an exception. 
    This is because an array's length must be known at compile time. 
    Here, the value of the variable "N" will be determined at runtime. 
    In other words, variables cannot be used to define the size of an array.
    */

    // However, the following program is valid −

    const M: usize = 20; 
    // pointer sized
    let arr8 = [1; M];
 
    print!("value {}",arr8[10]);



}
// Pass by value
fn update_array1(mut arr:[u32;3]){
    for i in 0..3{
        arr[i]=1;
    }
    println!("Inside update(Pass by value){:?}",arr);

    /*
    
    The value of an identifier prefixed with the const keyword is defined at compile time and cannot be changed at runtime. 
    usize is pointer-sized, thus its actual size depends on the architecture you are compiling your program for.
    
    */
}

// Pass by referance
fn update_array2(arr:&mut[u32;4]){
    for i in 0..=3{
        arr[i]=0;
    }
    println!("Inside update(Pass by referance){:?}",arr);
}
