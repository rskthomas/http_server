
use std::io::{Write, Read};
use std::net::TcpStream;
use crate::http::Request;
use crate::routes::Route;


pub fn handle_post(mut stream: TcpStream, request: Request, route: &Route) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}