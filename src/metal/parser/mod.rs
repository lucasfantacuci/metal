use super::message::{Request, Response};
use super::http::Method;

pub fn incomming_message(message: &[u8]) -> Request {
    let message = String::from_utf8_lossy(message);
    let mut request = Request {
        method: Method::Get
    };
    message.lines().for_each(|line| print_message_line(line, &mut request));
    request
}

pub fn output_message(response: &Response) -> &'static [u8; 98] {
    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
}

pub fn print_message_line(string_slice: &str, request: &mut Request) {
    println!("{}", string_slice);
}
