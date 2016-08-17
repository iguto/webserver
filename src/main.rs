use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;

const DOCUMENT_ROOT: &'static str = "/Users/iguto/tmp/webserver_document";

#[derive(Debug)]
pub enum Method {
    GET,
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
    let doc_root = Path::new(DOCUMENT_ROOT);

    let result = request_line.location.starts_with("/");
    println!("result: {}", result);
    let subpath = if result {
        let mut new_path = request_line.location.to_owned();
        new_path.remove(0);
        new_path
    } else {
        request_line.location
    };
    let location = doc_root.join(subpath);

    println!("real location: {:?}", location);
    if !location.is_file() {
        println!("location should be a path for file which exists.");
    } else {
        let mut file = File::open(location).expect("could not open file");
        println!("file: {:?}", file);
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("could not read from file");
        println!("file content: \n{}", content);
    }
}

// represents HTTP request line.
#[derive(Debug)]
pub struct RequestLine {
    pub method: Method,
    pub location: String,
}

impl RequestLine {
    fn new(raw_request_line: &str) -> RequestLine {
        let (method, location) = RequestLine::parse_request_line(&raw_request_line);
        println!("method: {:?}, location: {:?}", method, location);
        RequestLine {
            method: method,
            location: location,
        }
    }

    fn parse_request_line(raw_request_line: &str) -> (Method, String) {
        let words: Vec<&str> = raw_request_line.split(" ").collect();
        let method = RequestLine::detect_method(&words[0]).expect("invalid HTTP method");
        let location = words[1].trim();
        println!("w0 : {}  w1: {}", words[0], words[1]);
        (method, location.to_owned())
    }

    fn detect_method(target: &str) -> Option<Method> {
        if target.starts_with("GET") {
            return Some(Method::GET);
        }
        return None;
    }
}
