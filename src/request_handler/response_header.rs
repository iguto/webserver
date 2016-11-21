use super::request_line::*;

#[derive(Debug)]
pub struct ResponseHeader {
    pub content_type: String,
    pub body: String,
}

impl ResponseHeader {
    pub fn new(body: String) -> ResponseHeader {
        ResponseHeader {
            content_type: "text/html".to_owned(),
            body: body,
        }
    }

    pub fn render(&self, request_line: &RequestLine) -> String {
        format!("HTTP/{} {} OK\r\nContent-Type: text/html;\r\n\r\n{}", request_line.version, 200, self.body) // todo: [].join的な形にしたい
    }
}