extern crate regex;

use super::message::{Request, Response, Path};
use super::http::{Method, match_method};
use self::regex::Regex;

pub fn incomming_message(message: &[u8]) -> Result<Request, String> {

    let message = String::from_utf8_lossy(message).into_owned();

    let method : Method;
    let path : Path;

    let parse_http_method_result = parse_http_method(&message);
    if parse_http_method_result.is_ok() {
        method = parse_http_method_result.unwrap();
    } else {
        return Err(parse_http_method_result.unwrap_err());
    }

    let parse_path_called_result = parse_path_called(&message);
    if parse_path_called_result.is_ok() {
        path = parse_path_called_result.unwrap();
    } else {
        return Err(parse_path_called_result.unwrap_err());
    }

    let request = Request {
        method: method,
        path: path
    };

    println!("HTTP METHOD : {:?} ", &request.method);
    println!("HTTP URL : {} ", &request.path.path);

    Ok(request)
}

pub fn output_message(response: &Response) -> &'static [u8; 98] {
    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
}

pub fn parse_http_method(message: &String) -> Result<Method, String> {
    let regex = Regex::new(r"(^GET|^HEAD|^POST|^PUT|^DELETE|^CONNECT|^OPTIONS|^TRACE|^PATCH)").unwrap();
    let method = regex.find(message);
    match method {
        Some(value) => match_method(value.as_str()),
        None => Err(String::from("Invalid HTTP Method"))
    }
}

pub fn parse_path_called(message: &String) -> Result<Path, String> {
    let regex = Regex::new(r"[a-z0-9 -@]+").unwrap();
    let method = regex.find(message);
    match method {
        Some(value) =>  
            Ok(Path{ path: String::from(value.as_str())}),
        None => Err(String::from("Invalid HTTP Path"))
    }
}