use std::net::TcpListener;
// Using this: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Unwrap above stops the program if errors happen. For production we should
    // handle the errors instead.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}
