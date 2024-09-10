use std::collections::HashMap;
use std::string::String;


#[derive(Debug)]
pub enum Method{
    GET,
    POST,
    HEAD,
}

impl Method {
    pub fn from_string(method: &str) -> Method {
        match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "HEAD" => Method::HEAD,
            _ => panic!("Invalid method"),
        }
    }
    pub fn to_string(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::HEAD => "HEAD",
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,  
    pub resource_uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}


impl Request {
    pub fn construct(request_string:&Vec<String>) -> Request {
        //decompose first line into method, resource_uri, and version
        let request_line: Vec<&str> = request_string[0].split(" ").collect();
        let method = match request_line[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "HEAD" => Method::HEAD,
            _ => panic!("Invalid method"),
        };

        //headers are stored in a hashmap
        let mut headers = HashMap::new();
        for i in 1..request_string.len() {
            let header: Vec<&str> = request_string[i].split(": ").collect();
            headers.insert(header[0].to_string(), header[1].to_string());
        }
        Request {
            method,
            resource_uri: request_line[1].to_string(),
            version: request_line[2].to_string(),
            headers,
            body: String::new(),
        }

    }
    pub fn display(&self) {
        print!("{} request @ {}, version {} \n", self.method.to_string(), self.resource_uri, self.version);
    }

}

/*
 * Response struct
 * body is a reference to a String (html file) because we don't want to take ownership of the body
 */
pub struct Response {
    pub status_line: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
impl Response {
    pub fn format(&self) -> String {
        let mut response = String::new();
        response.push_str(&self.status_line);
        response.push_str("\r\n");
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}