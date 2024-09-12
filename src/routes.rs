use crate::handlers::error::{handle_method_not_allowed, handle_not_found};
use crate::handlers::get::handle_get;
use crate::handlers::post::handle_post;
use crate::http::{Method, Request};
use std::net::TcpStream;
use std::{collections::HashMap, hash::Hash};

pub struct Route {
    allows: AllowedMethods,
    file: String,
}
pub struct AllowedMethods {
    get: bool,
    post: bool,
}
enum HttpStatus {
    OK = 200,
    NotFound = 404,
    MethodNotAllowed = 405,
}

fn create_routes() -> HashMap<String, Route> {
    let mut routes = HashMap::new();
    routes.insert(
        // the "/"" route
        "/".to_string(),
        Route {
            allows: AllowedMethods {
                //allows get request
                get: true,
                post: false,
            },
            file: "resources/html/index.html".to_string(),
        },
    );
    routes.insert(
        "/about".to_string(),
        Route {
            allows: AllowedMethods {
                get: true,
                post: false,
            },
            file: "about.html".to_string(),
        },
    );
    routes.insert(
        "/comment".to_string(),
        Route {
            allows: AllowedMethods {
                get: true,
                post: true,
            },
            file: "about.html".to_string(),
        },
    );
    routes.insert(
        "/resume".to_string(),
        Route {
            allows: AllowedMethods {
                get: true,
                post: false,
            },
            file: "resources/html/resume.html".to_string(),
        },
    );
    routes.insert(
        "/styles.css".to_string(),
        Route {
            file: "resources/static/css/styles.css".to_string(),
            allows: AllowedMethods {
                get: true,
                post: false,
            },
        },
    );
    routes.insert(
        "/script.js".to_string(),
        Route {
            file: "resources/static/js/script.js".to_string(),
            allows: AllowedMethods {
                get: true,
                post: false,
            },
        },
    );
    

    routes
}

pub fn dispatcher(stream: TcpStream, request: Request)  {
    //for now, we will hardcode the routes
    let routes = create_routes();

    //debug compare the routes and request
    for i in routes.keys() {
        println!("Route: {}", i);
    }
    
    if routes.contains_key(&request.resource_uri) {
        //get the route
        let route = routes.get(&request.resource_uri).unwrap();
        //check if the method is allowed
        match request.method {
            Method::GET => {
                print!("Handling get request");
                if route.allows.get {
                    handle_get(stream, &route.file);
                } else {
                    handle_method_not_allowed(stream);
                }
            }
            Method::POST => {
                if route.allows.post {
                    handle_post(stream, request, route);
                } else {
                    handle_method_not_allowed(stream);
                }
            }
            _ => {
                handle_method_not_allowed(stream);
            }
        }
    } else {
        //handle not found
        print!("Handling not found");
        handle_not_found(stream);
    }
}
