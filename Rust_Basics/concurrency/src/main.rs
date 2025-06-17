//import the necessary modules
use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, Concurrency");

    //create a new thread
    // thread::spawn(|| {
    //     for i in 1..10 {
    //     println!("hi number {} from the spawned thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // //code executed by the main thread
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    let handle = thread::spawn(|| {
        for i in 1..10 {
           println!("hi number {} from the spawned thread!", i);
           thread::sleep(Duration::from_millis(1));
        }
     });
     for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
     }
     handle.join().unwrap();
}





















































