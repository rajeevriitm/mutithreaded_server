use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use trial::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        thread_pool.execute(Box::new(|| {
            let stream = stream.unwrap();
            handle_connectoin(stream);
        }));
    }
}
fn handle_connectoin(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Result: {}", String::from_utf8_lossy(&buffer));
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
