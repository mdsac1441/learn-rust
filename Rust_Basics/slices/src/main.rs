/*

A slice is a pointer to a block of memory. 
Slices can be used to access portions of data stored in contiguous memory blocks. 
It can be used with data structures like arrays, vectors and strings. 
Slices use index numbers to access portions of data. 
The size of a slice is determined at runtime.

Slices are pointers to the actual data. 
They are passed by reference to functions, which is also known as borrowing.

*/
// let sliced_value = &data_structure[start_index..end_index]

fn main() {
    println!("Hello, Slices");
    let n1="Bye";
    println!("length of string n1 is {}",n1.len());
    let n2:String = "Tutorials".to_string();
    println!("length of string n2 is {}",n2.len());
    let c1 = &n2[4..9]; 
    
    // fetches characters at 4,5,6,7, and 8 indexes
    println!("c1 is {}",c1);

    let data = [10,20,30,40,50];
    use_slice1(&data[1..4]);
    //this is effectively borrowing elements for a while

    let mut datas=[10,1,2,3,90];
    use_slice2(&mut datas[0..=4]);
    println!("{:?}",datas);
}

fn use_slice1(slice:&[i32]){
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
}

fn use_slice2(slice:&mut[i32]){

    println!("length of slice is {:?}",slice.len());
    println!("before mutate{:?}",slice);
    slice[1] = 900; // replaces 20 with 1010
}