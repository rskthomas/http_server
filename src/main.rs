use std::{
    string::String,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
//declaring modules
mod http;
mod config;
mod routes;
mod handlers{
    pub mod get;
    pub mod post;
    pub mod error;
}

use http::Request;
use routes::dispatcher;

//TODO: idea: website add a form that sends a post request to the server with a custom route and html file
//beware of the security implications of this

fn main() {
    //initial configuration
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {

    println!("Connection established with: {}", stream.peer_addr().unwrap());
    //1. parse the request
    let buf_reader = BufReader::new(&mut stream);

    let raw_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
   
    let request:Request = Request::construct(&raw_request);
    request.display();
    dispatcher(stream, &request);


}
