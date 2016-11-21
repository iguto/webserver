use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::str;

pub mod request_handler;

const DOCUMENT_ROOT: &'static str = "/Users/iguto/tmp/webserver_document"; // TODO: can be relative path

const BUFF_SIZE: usize = 128;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("server is listening: 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { thread::spawn(move || handle_client(stream)); },
            Err(e) => { println!("connection failed: {}", e); },
        }
    }
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut buf: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
    let mut s = String::new();
    let newline: u8 = '\n' as u8;
    println!("handle: {:?}", stream);
    loop {
        match stream.read(&mut buf) {
            Err(_) => {
                println!("could not read from stream");
                return;
            }
            Ok(0) => break,
            Ok(n) if n > 0 => {
                match buf.iter().position(|&e| e == newline) {
                    Some(index) => {
                        s.push_str(str::from_utf8(&buf[0..index]).expect("invalid data as utf8"));
                        break;
                    },
                    None => s.push_str(str::from_utf8(&buf[0..n]).expect("invalid data as utf8")),
                }
            }
            Ok(_) => unreachable!(),
        }
    }
    println!("{:?} : {}", stream, s);
    let request_handler = request_handler::RequestHandler::new(&s);

    let content = request_handler.response(DOCUMENT_ROOT);
    stream.write(content.as_bytes())
        .expect("could not write to stream");
}
