use std::net::TcpStream;
use std::io::prelude::*;

use log::{info, warn, error, debug};

pub struct HttpRequest {
    pub url: String,
    pub verb: String
}

impl HttpRequest{
    pub fn new(stream: &mut TcpStream) -> Result<HttpRequest, String>{
        let mut buffer = [0; 1024];
        let reqstr:String;
        match stream.read(&mut buffer) {
            Ok(l) => {
                info!("Read {} bytes from request", l);
                match String::from_utf8(buffer[0..l].to_vec()){
                    Ok(s) => {
                        reqstr = s;
                        debug!("Got this request {}", reqstr);
                    }
                    Err(e) => {
                        error!("Failed to parse request due to {}", e);
                        return Err(String::from(""));
                    }
                }
            }
            Err(_) => {
                error!("Can't read from the input stream");
                return Err(String::from(""))
            }
        }
        let mut lines = reqstr.split('\n');
        let firstline = lines.next();
        let mut url:String;
        let verb:String;
        match firstline {
            Some(fl) => {
                info!("Received request line: {}", fl);
                let mut words = fl.split_ascii_whitespace();
                let first_w = words.next();
                match first_w {
                    Some(v) => {
                        if v != "GET" {
                            error!("Only GET is supported but we got {}", v);
                            return Err(String::from(""));
                        }
                        verb = v.to_string();
                    }
                    None => {
                        error!("Request line is malformed. Abort processing!");
                        return Err(String::from(""));
                    }
                }
                let res = words.next();
                match res {
                    Some(u) => {
                        url = String::from(u);
                    }
                    None => {
                        error!("Request line is malformed. Abort processing!");
                        return Err(String::from(""));
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