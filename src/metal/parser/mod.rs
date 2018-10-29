extern crate regex;

use super::message::{Request, Response};
use super::http::{Method, match_method};
use self::regex::Regex;

pub fn incomming_message(message: &[u8]) -> Result<Request, String> {
    
    let message = String::from_utf8_lossy(message).into_owned();
    println!("{}", message);

    let method : Method;

    let parse_http_method_result = parse_http_method(&message);
    if parse_http_method_result.is_ok() {
        method = parse_http_method_result.unwrap();
    } else {
        return Err(parse_http_method_result.unwrap_err());
    }

    let request = Request {
        method: method
    };

    Ok(request)
}

pub fn output_message(response: &Response) -> &'static [u8; 98] {
    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
}

pub fn parse_http_method(message: &String) -> Result<Method, String> {
    let regex = Regex::new(r"(^GET|^HEAD|^POST|^PUT|^DELETE|^CONNECT|^OPTIONS|^TRACE|^PATCH)").unwrap();
    let method = regex.find(message);
    match method {
        Some(x) => match_method(x.as_str()),
        None => Err(String::from("Invalid HTTP Method"))
    }
}