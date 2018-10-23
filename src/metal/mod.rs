
use std::net::TcpListener;
use std::io::prelude::*;
use std::thread;

pub struct Metal;

impl Metal {
    pub fn listen(&self, address: String) {
        let listener = TcpListener::bind(address).expect("Unable to bind the server into this port");
        self.handle_incomming_messages(listener);
    }

    fn handle_incomming_messages(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            thread::spawn(move || {
                let mut buffer: [u8; 2048] = [0; 2048];
                stream.read(&mut buffer).unwrap();
                let request = Metal::parse_incomming_message(format!("{}", String::from_utf8_lossy(&buffer[..])));
                let response = Response;
                //self.dispatch_to_route(request, response);
                stream.write(Metal::parse_output_message(response));
            });
        }
    }

    fn parse_incomming_message(message: String) -> Request {
        println!("{}", message);
        let x = Request;
        x
    }

    fn parse_output_message(response: Response) -> &'static [u8; 98] {
        b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
    }
}

struct Request;

struct Response;