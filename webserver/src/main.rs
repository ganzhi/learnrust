use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use std::env;
use std::sync::Arc;

use config::WebServerConfig;

use num_cpus;
use threadpool::ThreadPool;

mod config;

fn main() {
    // Read configurations
    let args: Vec<String> = env::args().collect();
    let conf: Arc<WebServerConfig>;
    if args[1] == "--config" {
        conf = Arc::new(config::WebServerConfig::new(&args[2]));
        println!("Root directory is: {}", conf.root);
    } else {
        panic!();
    }
    
    let addr = "127.0.0.1:".to_owned() + &conf.listen_on.to_string();
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    
    let cpu_count = num_cpus::get();
    println!("Number of CPU is: {}", cpu_count);
    println!("Start {} threads", cpu_count);
    let pool = ThreadPool::new(cpu_count);

    for stream in listener.incoming().take(5) {
        let clone_conf = Arc::clone(&conf);
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream, clone_conf);            
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, conf: Arc<WebServerConfig>) {    
    println!("Arc strong count is {}", Arc::strong_count(&conf));

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // Prefix file name with root
    let mut root =  conf.root.to_owned();
    root.push_str(filename);

    let contents = fs::read_to_string(root).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}