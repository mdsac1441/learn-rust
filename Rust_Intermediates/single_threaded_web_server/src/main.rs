use std::{
    io::{prelude::*,BufReader},
    net::{TcpListener,TcpStream},
    fs,
};
fn main() {
    let listener=TcpListener::bind("127.0.0.1:3000").expect("Failed to bind");
    for stream in listener.incoming(){
        let stream=stream.expect("Unable to connect the server");
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let buffer_reader=BufReader::new(&mut stream);
    // let http_request:Vec<_>=buffer_reader
    // .lines()
    // .map(|result|result.expect("done"))
    // .take_while(|line|!line.is_empty())
    // .collect();  

    // println!("Request: {:#?}", http_request);
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // if http_request[0] == "GET / HTTP/1.1"{
    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("index.html").expect("unable to find thid file");
    // let length = contents.len();

    // let response =
    //     format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // stream.write_all(response.as_bytes()).expect("respose failed");
    // }else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );

    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    let request_line = buffer_reader.lines().next().unwrap().unwrap(); 
    // We’re only going to be looking at the first line of the HTTP request, 
    // so rather than reading the entire request into a vector, we’re calling next to get the first item from the iterator. 

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();


}

// Currently, our server runs in a single thread, meaning it can only serve one request at a time. 
// Let’s examine how that can be a problem by simulating some slow requests. 
// Then we’ll fix it so our server can handle multiple requests at once.
