
use std::net::TcpStream;
use std::io::{Write, Read};


pub fn handle_method_not_allowed(mut stream: TcpStream) {
    let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
pub fn handle_not_found(mut stream: TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}