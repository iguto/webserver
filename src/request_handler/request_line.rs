use std::fmt;


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