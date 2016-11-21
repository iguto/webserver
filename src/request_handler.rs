use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::fmt;

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

#[derive(Debug)]
pub enum HTTPVersion {
    PointNine,    // 0.9
    ONE,          // 1.0
    OnePointOne,  // 1.1
}

impl fmt::Display for HTTPVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HTTPVersion::PointNine =>  write!(f, "0.9"),
            HTTPVersion::ONE => write!(f, "1.0"),
            HTTPVersion::OnePointOne => write!(f, "1.1"),
        }
    }
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
    pub version: HTTPVersion,
}

impl RequestLine {
    pub fn new(raw_request_line: &str) -> RequestLine {
        let (method, location, version) = RequestLine::parse_request_line(&raw_request_line);
        println!("method: {:?}, location: {:?}", method, location);
        RequestLine {
            method: method,
            location: location,
            version: version,
        }
    }

    fn parse_request_line(raw_request_line: &str) -> (Method, String, HTTPVersion) {
        let words: Vec<&str> = raw_request_line.split(" ").collect();
        println!("w0: {}  w1: {} w2: {}", words[0], words[1], words[2]);

        let method = RequestLine::detect_method(&words[0]).expect("invalid HTTP method");
        let location = words[1].trim();
        let version = RequestLine::detect_version(&words[2].trim());
        (method, location.to_owned(), version)
    }

    fn detect_method(target: &str) -> Option<Method> {
        if target.starts_with("GET") {
            return Some(Method::GET);
        }
        return None
    }

    fn detect_version(string: &str) -> HTTPVersion {
        match (string.starts_with("HTTP/"), string.split("/").nth(1).unwrap()) {
            (true, "1.0") => HTTPVersion::ONE,
            (true, "1.1") => HTTPVersion::OnePointOne,
            _ => HTTPVersion::PointNine,
        }
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
        match self.request_line.version {
            HTTPVersion::PointNine => content,
            _ => {
                let header = ResponseHeader::new(content);
                header.render(&self.request_line)
            },
        }
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

#[derive(Debug)]
pub struct ResponseHeader {
    pub content_type: String,
    pub body: String,
}

impl ResponseHeader {
    fn new(body: String) -> ResponseHeader {
        ResponseHeader {
            content_type: "text/html".to_owned(),
            body: body,
        }
    }

    fn render(&self, request_line: &RequestLine) -> String {
        format!("HTTP/{} {} OK\r\nContent-Type: text/html;\r\n\r\n{}", request_line.version, 200, self.body) // todo: [].join的な形にしたい
    }
}