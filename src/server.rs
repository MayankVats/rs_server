use std::{net::TcpListener, io::{Write, Read}, convert::TryFrom};
use crate::http::{Request, Response, StatusCode, response, ParseError};

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    println!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  addr: String,
}

impl Server {
  // This is considered as constructor to the Server struct.
  // Also, technically it is an associated function, a method which can be directly called on struct not on the instance of the struct.
  // It does not accept 'self' as its first parameter
  pub fn new(addr: String) -> Self {
      Self {
          addr
      }
  }

  pub fn run(self, mut handler: impl Handler) {
    let listener = TcpListener::bind(&self.addr).unwrap();
    println!("Server is running on {}", self.addr);

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
              let response = match Request::try_from(&buffer[..]) {
                Ok(request) => {
                  handler.handle_request(&request)
                },
                Err(err) => {
                  handler.handle_bad_request(&err)
                }
              };

              if let Err(e) = response.send(&mut stream) {
                println!("Failed to send response: {}", e);
              }
            },
            Err(e) => {
              println!("Failed to read from connection: {}", e);
            }
          }
        },
        Err(err) => {
          println!("Failed to establish connection {}", err);
          continue;
        }
      }
    }
  }
}