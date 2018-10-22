
use std::net::TcpListener;
use std::io::prelude::*;

pub struct Metal;

impl Metal {
    pub fn listen(address: String) {
        let listener = TcpListener::bind(address).expect("Unable to bind the server into this port");
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        }
    }
}