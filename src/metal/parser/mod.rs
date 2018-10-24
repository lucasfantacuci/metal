pub fn parse_incomming_message(message: &[u8]) -> super::message::Request {
        
    let message = String::from_utf8_lossy(message);
    
    let x = super::message::Request {
        method: super::http::Method::GET
    };

    message.lines().for_each(|line| print_message_line(line));
    
    x
}

pub fn parse_output_message(response: super::message::Response) -> &'static [u8; 98] {
    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
}

pub fn print_message_line(string_slice: &str) {
    println!("{}", string_slice);
}
