mod http;
mod message;
mod parser;

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
                let request_parsed = parser::incomming_message(&buffer[..]);
                let response = message::Response::default();
                if request_parsed.is_err() {
                    // make changes to request and trown message error
                }else{
                    //self.dispatch_to_route(request, response);
                }
                stream.write(parser::output_message(&response));    
            });
        }
    }
}