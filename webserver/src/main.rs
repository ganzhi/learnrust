use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use std::env;
use std::sync::Arc;
use std::path::Path;
use log::{info, warn, error, debug};
use simplelog::*;

use config::WebServerConfig;

use num_cpus;
use threadpool::ThreadPool;
use indoc::*;

mod config;

fn main() {
    // Setup logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), fs::File::create("my_web_server.log").unwrap()),
        ]
    ).unwrap();

    // Read configurations
    let args: Vec<String> = env::args().collect();
    let conf: Arc<WebServerConfig>;
    if args[1] == "--config" {
        conf = Arc::new(config::WebServerConfig::new(&args[2]));
        info!("Root directory is: {}", conf.root);
    } else {
        panic!();
    }
    
    let addr = "127.0.0.1:".to_owned() + &conf.listen_on.to_string();
    info!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    
    let cpu_count = num_cpus::get();
    info!("Number of CPU is: {}", cpu_count);
    info!("Start {} threads", cpu_count);
    let pool = ThreadPool::new(cpu_count);

    for stream in listener.incoming().take(5) {
        let clone_conf = Arc::clone(&conf);
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream, clone_conf);            
        });
    }

    info!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, conf: Arc<WebServerConfig>) {    
    println!("Arc strong count is {}", Arc::strong_count(&conf));

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
                    return;
                }
            }
        }
        Err(_) => {
            error!("Can't read from the input stream");
            return;
        }
    }
    let mut lines = reqstr.split('\n');
    let firstline = lines.next();
    let url:String;
    match firstline {
        Some(fl) => {
            info!("Received request line: {}", fl);
            let mut words = fl.split_ascii_whitespace();
            let verb = words.next();
            match verb {
                Some(v) => {
                    if v != "GET" {
                        error!("Only GET is supported but we got {}", v)
                    }
                }
                None => {
                    error!("Request line is malformed. Abort processing!");
                    return;
                }
            }
            let res = words.next();
            match res {
                Some(u) => {
                    url = String::from(u);
                }
                None => {
                    error!("Request line is malformed. Abort processing!");
                    return;
                }
            }          
        }
        None => {
            error!("No line in the request. Abort processing");
            return;
        }
    }

    info!("Now start processing request for URL {}", url);

    // Prefix file name with root
    let root = Path::new(&conf.root);
    let path = root.join(url);
    if path.exists() {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        if path.is_dir() {
            let mut dir_content = String::from(
                indoc!{
                    "<!DOCTYPE html>
                    <html lang='en'>
                    <head>
                        <meta charset='utf-8'>
                        <title>Hello!</title>
                    </head>
                    <body>
                    "
                }
            );
            let rd = fs::read_dir(&conf.root).unwrap();
            for entry in rd {
                match entry {
                    Ok(e) => {
                        dir_content.push_str(e.path().to_str().unwrap());
                        dir_content.push_str("<br/>\n");
                    },
                    Err(err) => {
                        warn!("Found error {}", err);
                    }
                }
            }
            dir_content.push_str(indoc!{"
                        </body>
                    </html>"
                }
            );
            
            let response = format!("{}{}", status_line, dir_content);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            let contents = fs::read_to_string(path).unwrap();

            let response = format!("{}{}", status_line, contents);
        
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string(root.join("404.html")).unwrap();

        let response = format!("{}{}", status_line, contents);
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}