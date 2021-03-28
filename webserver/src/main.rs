use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
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
mod httppro;

use httppro::HttpRequest;

fn main() {
    // Setup logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
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

    let r : HttpRequest;
    match httppro::HttpRequest::new(&mut stream){
        Ok(req) => {
            r = req;
        }
        Err(e) => {
            error!("Failed to parse http request {}", e);
            return;
        }
    }

    info!("Now start processing request for URL {}", r.url);

    // Prefix file name with root
    let root = Path::new(&conf.root);
    debug!("Serving content from root: {}", &root.to_str().unwrap());
    let mut url = r.url.clone();
    if url.starts_with('/') {
        url.remove(0);
    }
    let path = root.join(url);
    debug!("Trying to find path {}", &path.to_str().unwrap());
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