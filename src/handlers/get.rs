
use std::fs;
use std::string::String;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;
use crate::http::{Request, Response};
use crate::handlers::error::handle_not_found;


pub fn handle_get(mut stream: TcpStream, file_path : &str) {

    let mut response = Response{
        status_line: String::new(),
        headers: HashMap::new(),
        body: String::new(),
    };

    //Check if resource exists from request uri
    let file = fs::read_to_string(file_path);

    match file {
        Ok(file) => {
            response.status_line = "HTTP/1.0 200 OK".to_string();
            response.headers.insert("Content-Length".to_string(), file.len().to_string());
            response.headers.insert("Content-Type".to_string(), get_content_type(file_path).to_string());
            response.body = file;
        }
        Err(_) => {
            handle_not_found(stream);
            return;
        }
    }

    stream.write(response.format().as_bytes()).unwrap();
    
    stream.flush().unwrap();
}

fn get_content_type(file_path: &str) -> &str {
    if file_path.ends_with(".html") {
        "text/html"
    } else if file_path.ends_with(".css") {
        "text/css"
    } else if file_path.ends_with(".js") {
        "application/javascript"
    } else {
        "text/plain"
    }
}