use tokio::task;
use std::rc::Rc;
pub async fn add(a:u32,b:u32)->u32{
    a+b
}
#[tokio::main]
async fn main() {
    let add=add(2,8).await;
    println!("{}",add);
    println!("Hello, world!");
    let v :Vec<u32>=vec![1,2,3,4,5];
    task::spawn(async move{
        println!("vec{:?}",v);
    });

    // tokio spawn:
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        task::yield_now().await;
    });
}
