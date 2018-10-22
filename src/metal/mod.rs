
use std::net::TcpListener;

pub struct Metal;

impl Metal {
    pub fn listen(address: String) {
        let listener = TcpListener::bind(address).expect("Unable to bind the server into this port");
        for stream in listener.incoming() {
            print!("Hello metal")   
        }
    }
}