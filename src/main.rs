#![allow(unused_variables)]
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let resource_names = ["index.js"];
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("Reading failed");

    let mut filename = "404.html";
    let mut status_line  = "HTTP/1.1 404 NOT FOUND \r\n\r\n";


    for resource in &resource_names {
        let resource_get = format!("GET /{} HTTP/1.1\r\n", resource);
        let resource_get_bytes = resource_get.as_bytes();
        if buffer.starts_with(resource_get_bytes){
            filename = resource;
            status_line = "HTTP/1.1 200 OK \r\n\r\n";
        }
        else if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
            filename = "hello.html";
            status_line = "HTTP/1.1 200 OK \r\n\r\n";
        }

    }
    let content = fs::read_to_string(filename).expect("File not found");
    let response = format!("{}{}", status_line, content);


    // println!("{}", String::from_utf8_lossy(&buffer)); //prints the request
    stream.write(response.as_bytes()).unwrap();
    stream.flush().expect("Flush wrong: Error");
}

