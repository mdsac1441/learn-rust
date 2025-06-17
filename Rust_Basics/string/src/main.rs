fn main() {
    /* 
    The String data type in Rust can be classified into the following −
    *String Literal(&str)
    *String Object(String)
    */
    // String Literal(&str)
    let company:&str="TutorialsPoint";
    let location:&str = "Hyderabad";
    println!("company is : {}\n location :{}",company,location);

    let company:&'static str = "TutorialsPoint";
    let location:&'static str = "Hyderabad";
    println!("company is : {} location :{}\n",company,location);

    // String Object

    let empty_string = String::new();
    println!("length is {}",empty_string.len());
 
    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());

    let mut z = String::new();
    z.push('a');z.push('b');
    z.push_str("\nMd Sharique Ahmed");
    println!("{}",z);

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }
    let name1 = "Hello TutorialsPoint , Hello!".to_string();
    print_type_of(&name1);


    let name2 = "Hello TutorialsPoint , 
    Hello!".to_string();         //String object
    let name3 = name2.replace("Hello","Howdy");    //find and replace
    println!("{}",name3);

    let example_string = String::from("example_string");
    print_literal(example_string.as_str());
    print_type_of(&print_literal);
 
 fn print_literal(data:&str ){
    println!("displaying string literal {}",data);
 }

 let msg = "Tutorials Point has good t
 utorials".to_string();
 let mut i = 1;
 
 for token in msg.split_whitespace(){
    println!("token {} {}",i,token);
    i+=1;
 }

 let fullname = "Kannan,Sudhakaran,Tutorialspoint";

 for token in fullname.split(","){
    println!("token is {}",token);
 }

 //store in a Vector
 println!("\n");
 let tokens:Vec<&str>= fullname.split(",").collect();
 println!("firstName is {}",tokens[0]);
 println!("lastname is {}",tokens[1]);
 println!("company is {}",tokens[2]);

 let n1 = "Tutorials".to_string();

 for n in n1.chars(){
    println!("{}",n);
 }
  
 let n2 = "Point".to_string();

 let n3 = n1 + &n2; // n2 reference is passed
 println!("{}",n3);

 let number = 2020;
 let number_as_string = number.to_string(); 
 
 // convert number to string
 println!("{}",number_as_string);
 println!("{}",number_as_string=="2020");
 let n4 = "Tutorials".to_string();
 let n5 = "Point".to_string();
 let n6 = format!("{} {}",n4,n5);
 println!("{}",n6);

}
