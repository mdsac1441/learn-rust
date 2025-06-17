pub fn fact(num:u32)->u32{
    if num==0{
        1
    }else{
        num*fact(num-1)
    }
}