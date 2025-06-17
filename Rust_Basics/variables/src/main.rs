fn main() {
    let college_fees = 25_000;
    let salary:f64 = 35_000.00;
    println!("fees is {} and salary is {}",college_fees,salary);
    /* Immutable
   By default, variables are immutable 
    */
//     let fees = 25_000;
//     println!("fees is {} ",fees);
//     fees = 35_000;
//     println!("fees changed is {}",fees);
let mut fees:i32 = 25_000;
println!("fees is {} ",fees);
fees = 35_000;
println!("fees changed is {}",fees);

}
