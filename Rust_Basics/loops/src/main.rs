fn main() {
    println!("Loops Start");

    // loop{
    //     println!("infinite");
    // }

    let mut i=0;
    // loop{
    //     println!("{}",i);
    //     if i==9{
    //         break;
    //     }
    //     i+=1;
    // }

    loop {

        i+=1;
        if i==7 {
           continue;
        }
        println!("i={}",i);
        if i==15{
            break;
        }
    }


    for x in 1..11{
        if x%2==0{
            continue;
        }
        println!("{}",x);
    }

    let mut y=10;
    while y > 0{
        println!("{}",y);
        y-=1;
    }
    
    
}
