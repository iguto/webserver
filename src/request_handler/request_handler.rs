use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};

use super::request_line::*;
use super::response_header::*;

const NOT_FOUND_HTML: &'static str = r#"
<html>
<head>
    <title>Not Found</title>
</head>
<body>
<h1>Not Found</h1>
</body>
"#;

enum RequestError {
    NotFound,
    Forbidden,
}

pub struct RequestHandler {
    pub request_line: RequestLine,
}

impl RequestHandler {
    pub fn new(raw_request_line: &str) -> RequestHandler {
        RequestHandler { request_line: RequestLine::new(raw_request_line) }
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
