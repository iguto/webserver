use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug)]
pub enum Method {
    GET
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("connection failed: {}", e);
            }
        }
    }
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut s = String::new();
    println!("handle: {:?}", stream);
    stream.read_to_string(&mut s)
        .expect("could not read from stream");
    println!("{:?} : {}", stream, s);
    let request_line = RequestLine::new(&s);
    println!("request line: {:?}", request_line);

}

// represents HTTP request line.
#[derive(Debug)]
pub struct RequestLine<'a> {
    pub method: Method,
    pub location: &'a Path
}

impl<'a> RequestLine<'a> {
    fn new(raw_request_line: &'a str) -> RequestLine<'a> {
        let (method, location) = RequestLine::parse_request_line(&raw_request_line);
        println!("method: {:?}, location: {:?}", method, location);
        RequestLine { method: method, location: location }
    }

    fn parse_request_line(raw_request_line: &'a str) -> (Method, &'a Path) {
        let words: Vec<&str> = raw_request_line.split(" ").collect();
        let method = RequestLine::detect_method(&words[0])
            .expect("invalid HTTP method");
       println!("w0 : {}  w1: {}", words[0], words[1]);
       (method, Path::new(words[1]))
    }

    fn detect_method(target: &str) -> Option<Method> {
        if target.starts_with("GET") {
            return Some(Method::GET);
        }
        return None;
    }
}
// ~/tmp/webserver_document
