use std::{io::Error, net::TcpStream};
use std::io::prelude::*;

use log::{info, error, debug};

pub struct HttpRequest {
    pub url: String,
    pub verb: String
}

impl HttpRequest{
    pub fn new(stream: &mut TcpStream) -> Result<HttpRequest, Error>{
        let mut buffer = [0; 1024];
        let reqstr = match stream.read(&mut buffer) {
            Ok(l) => {
                info!("Read {} bytes from request", l);
                match String::from_utf8(buffer[0..l].to_vec()){
                    Ok(s) => s,
                    Err(e) => {
                        let msg = format!("Failed to parse request due to {}", e);
                        error!("{}", msg);
                        println!("{:x?}", &buffer[0..l]);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            msg,
                        ));
                    }
                }
            }
            Err(e) => {               
                error!("{}", e);
                return Err(e);
            }
        };
        debug!("Got this request {}", reqstr);

        let mut lines = reqstr.split('\n');
        let firstline = lines.next();
        let url:String;
        let verb:String;
        match firstline {
            Some(fl) => {
                info!("Received request line: {}", fl);
                let mut words = fl.split_ascii_whitespace();                
                match words.next() {
                    Some(v) => {
                        if v != "GET" {
                            let msg = format!("Only GET is supported but we got {}", v);
                            error!("{}", msg);
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                msg,
                            ));
                        }
                        verb = v.to_string();
                    }
                    None => {
                        let msg = format!("Request line is malformed. Abort processing!");
                        error!("{}", msg);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            msg,
                        ));
                    }
                }
                
                url = match words.next() {
                    Some(u) => String::from(u),                    
                    None => {
                        let msg = format!("Request line is malformed. Abort processing!");
                        error!("{}", msg);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            msg,
                        ));
                    }
                }          
            }
            None => {
                let msg = "No line in the request. Abort processing";
                error!("{}", msg);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    msg,
                ));
            }
        }
    
        return Ok(HttpRequest{
            url,
            verb
        })
    }
}

pub struct HttpResponse {
    pub response: String,
}

impl HttpResponse {
    pub fn send_response(&mut self, stream: &mut TcpStream) -> std::io::Result<()>{
        let bytes = self.response.as_bytes();
        let mut pos = 0;
        while pos < bytes.len()-1 {
            let l = stream.write(&bytes[pos..])?;
            if l == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write the buffered data",
                ));
            }
            pos += l;
            stream.flush()?;
        }
        Ok(())
    }
}