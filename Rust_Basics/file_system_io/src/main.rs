use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::env::{args,Args};


fn main() {

    println!("Hello, File System");

    // Write
    let mut file = std::fs::File::create("src/text.txt").expect("create failed");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
    println!("data written to file" );



    // Read
    let mut file2 = std::fs::File::open("src/text.txt").unwrap();
    let mut contents = String::new();
    file2.read_to_string(&mut contents).unwrap();
    print!("{}", contents);


    let mut file3 = OpenOptions::new().append(true).open("src/data.txt").expect(
        "cannot open file");
     file3.write_all(" Hello World".as_bytes()).expect("write failed");
     file3.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
     println!("\nfile append success");


    let mut command_line: Args = args();
    command_line.next().unwrap();
    // skip the executable file name
    // accept the source file
    let source = command_line.next().unwrap();
    // accept the destination file
    let destination = command_line.next().unwrap();
    let mut file_in = File::open(source).unwrap();
    let mut file_out = File::create(destination).unwrap();
    let mut buffer = [0u8; 4096];
    loop {
       let nbytes = file_in.read(&mut buffer).unwrap();
       file_out.write(&buffer[..nbytes]).unwrap();
       if nbytes < buffer.len() { break; }
    }
}
