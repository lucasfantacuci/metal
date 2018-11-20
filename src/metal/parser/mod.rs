extern crate regex;

use super::message::{Request, Response, Path, Header, Headers, Cookie, Cookies};
use super::http::{Method, match_method};
use self::regex::Regex;

pub fn incomming_message(message: &[u8]) -> Result<Request, String> {

    let message = String::from_utf8_lossy(message).into_owned();

    let method : Method;
    let path : Path;
    let headers : Headers;
    let cookies : Cookies;

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

    let parse_headers_result = parse_headers(&message);
    if parse_headers_result.is_ok() {
        headers = parse_headers_result.unwrap();
    } else {
        return Err(parse_headers_result.unwrap_err());
    }

    let parse_cookies_result = parse_cookies(&message);
    if parse_cookies_result.is_ok() {
        cookies = parse_cookies_result.unwrap();
    } else {
        return Err(parse_cookies_result.unwrap_err());
    }

    let request = Request {
        method: method,
        path: path,
        headers: headers,
        cookies: cookies
    };

    println!("{:?}", request);
    println!("\r\n{}\r\n", message);

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

pub fn parse_headers(message: &String) -> Result<Headers, String> {
    //can improve heards by working with immutable values
    let mut headers = Headers::default();
    let regex = Regex::new(r"[ -~]+: [ -~]+").unwrap();
    for unparsed_header in regex.find_iter(message){
        let header = parse_header(unparsed_header.as_str());
        headers.add_header(header);
    }
    Ok(headers)
}

pub fn parse_header(unparsed_header: &str) -> Header {
    let splited_header : Vec<&str> = unparsed_header.splitn(2, ": ").collect();
    Header {
        name: String::from(splited_header[0]),
        value: String::from(splited_header[1])
    }
}

fn parse_cookies(message: &String) -> Result<Cookies, String> {
    let mut cookies = Cookies::default();
    let regex = Regex::new(r"cookie: [ -~]+").unwrap();
    let cookie_line = regex.find(message);
    match cookie_line {
        Some(value) => {
            let cookie_line = value.as_str();
            let cookie_line = cookie_line.replace("cookie: ", "");
            let unparsed_cookies : Vec<&str> = cookie_line.split("; ").collect();
            for unparsed_cookie in unparsed_cookies.into_iter() {
                let cookie = parse_cookie(unparsed_cookie);
                cookies.add_cookie(cookie);
            }
            Ok(cookies)
        }
        None => Ok(cookies)
    }
}

fn parse_cookie(unparsed_cookie: &str) -> Cookie {
    let splited_cookie : Vec<&str> = unparsed_cookie.splitn(2, "=").collect();
    Cookie {
        name: String::from(splited_cookie[0]),
        value: String::from(splited_cookie[1])
    }
}