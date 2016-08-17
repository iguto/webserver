use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use std::fs::File;

const DOCUMENT_ROOT: &'static str = "/Users/iguto/tmp/webserver_document";
const NOT_FOUND_HTML: &'static str = r#"
<html>
<head>
    <title>Not Found</title>
</head>
<body>
<h1>Not Found</h1>
</body>
"#;

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
    let request_handler = RequestHandler::new(request_line);

    let content = request_handler.response(DOCUMENT_ROOT);
    stream.write(content.as_bytes())
        .expect("could not write to stream");
}

enum RequestError {
    NotFound,
    Forbidden,
}

pub struct RequestHandler {
    pub request_line: RequestLine,
}

impl RequestHandler {
    fn new(request_line: RequestLine) -> RequestHandler {
        RequestHandler { request_line: request_line }
    }

    fn response(&self, root: &str) -> String {
        let mut file = match self.file(root) {
            Err(RequestError::NotFound) => return NOT_FOUND_HTML.to_owned(),
            Err(RequestError::Forbidden) => return "Forbidden".to_owned(),
            Ok(f) => f,
        };
        println!("file: {:?}", file);
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("could not read from file");
        println!("file content: \n{}", content);
        content
    }

    fn file(&self, root: &str) -> Result<File, RequestError> {
        let path = self.file_path(root);
        if !path.exists() || !path.is_file() {
            Err(RequestError::NotFound)
        } else {
            match File::open(path) {
                Ok(f) => Ok(f),
                Err(_) => Err(RequestError::Forbidden),
            }
        }
    }

    fn file_path(&self, root: &str) -> PathBuf {
        let path = if self.request_line.location.starts_with("/") {
            let mut new_path = self.request_line.location.to_owned();
            new_path.remove(0);
            new_path
        } else {
            self.request_line.location.to_owned()
        };
        let doc_root = Path::new(&root).to_owned();
        doc_root.join(path).to_owned()
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
