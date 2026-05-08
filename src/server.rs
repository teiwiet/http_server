use std::net::TcpListener;
use std::convert::TryFrom;
use crate::http::request::Request;
use std::io::Read;
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(ip_address: String) -> Server {
        Server {
            address: ip_address,
        }
    }
    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(self.address).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream,address)) => {
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request : {}",String::from_utf8_lossy(&buffer));
                            Request::try_from(&buffer[..]);
                        },
                        Err(e) => println!("Failed to read from connection: {}",e)
                    }
                },
                Err(e) => println!("Failed to establish a connection : {}",e),
            }
        }
    }
}
