use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>IT WORKS!</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                eprintln!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to establish a connection: {}", e),
            }
        }
    }
}
