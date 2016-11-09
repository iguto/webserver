use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};


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

enum RequestError {
    NotFound,
    Forbidden,
}
// represents HTTP request line.
#[derive(Debug)]
pub struct RequestLine {
    pub method: Method,
    pub location: String,
}

impl RequestLine {
    pub fn new(raw_request_line: &str) -> RequestLine {
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

pub struct RequestHandler {
    pub request_line: RequestLine,
}

impl RequestHandler {
    pub fn new(request_line: RequestLine) -> RequestHandler {
        RequestHandler { request_line: request_line }
    }

    pub fn response(&self, root: &str) -> String {
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
