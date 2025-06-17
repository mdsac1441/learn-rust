/******************************************************************************
Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, Java, PHP, Ruby, Perl,
C#, OCaml, VB, Swift, Pascal, Fortran, Haskell, Objective-C, Assembly, HTML, CSS, JS, SQLite, Prolog.
Code, Compile, Run and Debug online from anywhere in world.

*******************************************************************************/
fn main() {
    println!("Hello Number Patterns World");
    
    const n:u32=5;
    
    for i in 0..n{
        for j in (1..i+2){
            print!("{} ",j)
        }
        println!();
    }
    
    println!("Patterns:1");
    
     for i in (0..n).rev(){
        for j in (1..i+2){
            print!("{} ",j)
        }
        println!();
    }
    
    println!("Patterns:2");
     
     for i in 0..n{
        for j in (i+1..n+1){
            print!("{} ",j-i)
        }
        println!();
    } 
    
    println!("Patterns:2 2nd method");
    
    for i in 0..n{
        for j in (i+1..n+1){
            print!("{} ",j)
        }
        println!();
    } 
    
    println!("Patterns:3");
    
      for i in 0..n{
       for j in (i..n){
            print!("  ");
       }
        for j in (1..i+2){
            print!("{} ",j)
        }
        println!();
    }
    
    println!("Patterns:4");
    
    for i in (0..n).rev(){
       for j in (i..n){
            print!("  ");
       }
        for j in (1..i+2){
            print!("{} ",j)
        }
        println!();
    }
    
    println!("Patterns:5");
    
    for i in 0..n{
       for j in (0..i+1){
            print!("  ");
       }
        for j in (i+1..n+1){
            print!("{} ",j-i)
        }
        println!();
    }
    
    println!("Patterns:5 2nd method");
    
}
