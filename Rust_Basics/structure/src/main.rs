struct Student{
    roll_num:u32,
    name:String,
    age:u32,
}

//define dimensions of a rectangle
struct Rectangle {
    width:u32, height:u32
 }

 /*
 
 Methods are declared outside the structure block. 
 The impl keyword is used to define a method within the context of a structure. 
 The first parameter of a method will be always self, 
 which represents the calling instance of the structure. 
 To invoke a method, we need to first instantiate the structure. 
 The method can be called using the structure's instance.
 
 */
 // Method in Structure
 //logic to calculate area of a rectangle
 impl Rectangle {
    fn area(&self)->u32 {
       //use the . operator to fetch the value of a field via the self keyword
       self.width * self.height
    }
 }

 /*
 
 Static Method in Structure
Static methods can be used as utility methods. 
These methods exist even before the structure is instantiated. 
Static methods are invoked using the structure's name and can be accessed without an instance. 
Unlike normal methods, a static method will not take the &self parameter.
A static method like functions and other methods can optionally contain parameters.
 
 */

 //declare a structure
struct Point {
    x: i32,
    y: i32,
 }
 impl Point {
    //static method that creates objects of the Point structure
    fn get_instance(x:&mut i32, y:&mut i32) -> Point {
       Point { x: *x+10, y: *y+10 }
    }
    //display values of the structure's field
    fn display(&self){
       println!("x ={} y={}",self.x,self.y );
    }
 }


fn main() {
    println!("Hello, Structure");

    let mut s1=Student{
        roll_num:1,
        name:String::from("Md Sharique Ahmed"),
        age:23

    };

    let s2=Student{
        roll_num:2,
        name:String::from("Chirag"),
        age:25
    };

    println!("Roll_Number is :{} \nName is {} \nage is {}\n",s1.roll_num,s1.name,s1.age);
    s1.age=24;
    println!("Roll_Number is :{} \nName is {} \nupdate age is {}\n",s1.roll_num,s1.name,s1.age);
    println!("Call from Function\n");
    // print(s1);
    // print(s2);
    let older_student=who_is_elder(s1,s2);
    println!("Oder Student name with age : {} -> {}\n",older_student.name,older_student.age);


    // Method in Structure
    // instanatiate the structure
   let small = Rectangle {
    width:10,
    height:20
 };
 //print the rectangle's area
 println!("width is {} height is {} area of Rectangle 
 is {}\n",small.width,small.height,small.area());

    // Invoke the static method
    let p1 = Point::get_instance(&mut 10,&mut 20);
    p1.display();

}

// fn print(stu:Student){
//     println!("Roll_Number is :{} \nName is {} \nage is {}\n",stu.roll_num,stu.name,stu.age);
// }

fn who_is_elder(st1:Student,st2:Student)->Student{
    if st1.age > st2.age{
        st1
    }else{
        st2
    }
}
