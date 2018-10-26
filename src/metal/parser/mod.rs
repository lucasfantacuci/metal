extern crate regex;

use super::message::{Request, Response};
use super::http::{Method, match_method};
use self::regex::Regex;

pub fn incomming_message(message: &[u8]) -> Result<Request, &'static str> {
    
    let message = String::from_utf8_lossy(message).into_owned();
    
    let method = Method::Get;

    //tratar chamada abaixo
    parse_method(&message);

    let request = Request {
        method: method
    };

    Ok(request)
}

pub fn output_message(response: &Response) -> &'static [u8; 98] {
    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
}

pub fn parse_method(message: &String) -> Result<Method, &'static str> {
    let regex = Regex::new(r"(^GET|^HEAD|^POST|^PUT|^DELETE|^CONNECT|^OPTIONS|^TRACE|^PATCH)").unwrap();
    let method = regex.find(message);
    match method {
        Some(x) => match_method(x.as_str()),
        None => Err("Invalid HTTP Method")
    }
}