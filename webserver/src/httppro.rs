use std::net::TcpStream;
use std::io::prelude::*;

use log::{info, error, debug};

pub struct HttpRequest {
    pub url: String,
    pub verb: String
}

impl HttpRequest{
    pub fn new(stream: &mut TcpStream) -> Result<HttpRequest, String>{
        let mut buffer = [0; 1024];
        let reqstr = match stream.read(&mut buffer) {
            Ok(l) => {
                info!("Read {} bytes from request", l);
                match String::from_utf8(buffer[0..l].to_vec()){
                    Ok(s) => s,
                    Err(e) => {
                        let msg = format!("Failed to parse request due to {}", e);
                        error!("{}", msg);
                        return Err(msg);
                    }
                }
            }
            Err(_) => {
                let msg = format!("Can't read from the input stream");                
                error!("{}", msg);
                return Err(msg);
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
                            return Err(msg);
                        }
                        verb = v.to_string();
                    }
                    None => {
                        let msg = format!("Request line is malformed. Abort processing!");
                        error!("{}", msg);
                        return Err(msg);
                    }
                }
                
                url = match words.next() {
                    Some(u) => String::from(u),                    
                    None => {
                        let msg = format!("Request line is malformed. Abort processing!");
                        error!("{}", msg);
                        return Err(msg);
                    }
                }          
            }
            None => {
                error!("No line in the request. Abort processing");
                return Err(String::from(""));
            }
        }
    
        return Ok(HttpRequest{
            url,
            verb
        })
    }
}