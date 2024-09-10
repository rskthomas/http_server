
pub mod get;
pub mod post;
pub mod error;

use std::collections::HashMap;
use std::net::TcpStream;
use crate::http::{Request, Method};
use crate::handlers::get::handle_get;
