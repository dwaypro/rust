use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

// use serialize::json;

use std::thread;
use std::time::Duration;


// use hyper::header::{Headers, AccessControlAllowOrigin};

// let mut headers = Headers::new();
// headers.set(
//     AccessControlAllowOrigin::Any
// );

// use webserver::ThreadPool;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("Connection established!");

    }
}

fn handle_connection(mut stream: TcpStream){

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    println!("buffer ==> ");
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}