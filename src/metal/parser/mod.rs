extern crate regex;

use super::message::{Request, Response};
use super::http::Method;
use self::regex::Regex;

pub fn incomming_message(message: &[u8]) -> Request {
    
    let message = String::from_utf8_lossy(message).into_owned();
    
    parse_method(&message);

    let mut request = Request {
        method: Method::Get
    };

    request
}

pub fn output_message(response: &Response) -> &'static [u8; 98] {
    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
}

pub fn parse_method(message: &String){
    let regex = Regex::new(r"(^GET|^HEAD|^POST|^PUT|^DELETE|^CONNECT|^OPTIONS|^TRACE|^PATCH)").unwrap();
    let method = regex.find(message).expect("asd");
    println!("PARSED HTTP METHOD THAT IS => {}", method.as_str());
}