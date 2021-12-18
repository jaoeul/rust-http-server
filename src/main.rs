use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    // Returns a Result<T, E>, which is why we need to call unwrap to get to the
    // listener. Unwrap terminates the program if the returned value is an
    // error.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool     = ThreadPool::new(4);

    // incoming() returns an iterator of TcpStreams.
    for stream in listener.incoming().take(2) {
        // We call unwrap here in case that the stream returned by incoming()
        // was not a successfull connection attempt.
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get   = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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

    // The from_utf8_lossy takes a &[u8] (byte slice) and returs a string from
    // it. It is "lossy" since it will replace any unknown utf-8 characters with
    // �.
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
