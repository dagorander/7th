use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// Using this: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Unwrap above stops the program if errors happen. For production we should
    // handle the errors instead.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// Next up:
// Multithread with: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}