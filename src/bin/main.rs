use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use web_server_1::ThreadPool;

fn main() {
    let host = "localhost";
    let port = 7878;

    let url = format!("{}:{}", host, port);

    let listener = TcpListener::bind(url).unwrap();

    let pool_size = 10;

    let pool = ThreadPool::new(pool_size);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| { handle_stream(stream); });
    }
}

fn handle_stream(mut stream: TcpStream) {
    const BYTES: usize = 1024;
    let mut buffer = [0; BYTES];

    stream.read(&mut buffer).unwrap();

    const STATUS_HTTP: &str = "HTTP/1.1";

    let get = b"GET / HTTP/1.1\r\n";

    let (status_code, reason_phrase, file) = if buffer.starts_with(get) {
        (200, "OK", "nice.html")
    } else {
        (404, "NOT FOUND", "error.html")
    };

    const CRLF: &str = "\r\n";

    let contents = fs::read_to_string(file).unwrap();

    let response = format!("{} {} {}{}Content-Length: {}{}{}{}", STATUS_HTTP, status_code, reason_phrase, CRLF, contents.len(), CRLF, CRLF, contents);

    let response = response.as_bytes();

    stream.write(response).unwrap();
    stream.flush().unwrap();

    // println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
}