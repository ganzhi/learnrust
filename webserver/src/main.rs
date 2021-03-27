use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use std::env;
use std::sync::Arc;
use std::path::Path;
use log::{info, warn, error};
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
    match stream.read(&mut buffer) {
        Ok(s) => {
            info!("Read {} bytes from request", s);
            let s = String::from_utf8_lossy(&buffer[0..s]);
            info!("Got tis request {}", s)
        }
        Err(_) => {
            error!("Can't read from the input stream");
            return;
        }
    }


    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // Prefix file name with root
    let mut root =  conf.root.to_owned();
    root.push_str(filename);
    if Path::new(&root).exists() {
        let contents = fs::read_to_string(root).unwrap();

        let response = format!("{}{}", status_line, contents);
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
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
    }
}