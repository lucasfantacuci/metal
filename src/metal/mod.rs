
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
                println!("{}", String::from_utf8_lossy(&buffer[..]));
            });
        }
    }
}