use std::{fs::File, io::Read};

#[derive(Clone)]
pub struct WebServerConfig {
    pub root: String,
    pub listen_on: u32
}

impl WebServerConfig {
    pub fn new(path: &str) -> WebServerConfig {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        let _res = file.read_to_string(&mut contents).unwrap();
        let config: toml::Value = toml::from_str(&contents).unwrap();
        let root = config["root"].as_str().unwrap();
        let root = root.to_string();

        let listen_port = config["listen_on"].as_integer();
        let port:u32;
        match listen_port {
            Some(p) => {
                port = p as u32
            }
            None => {
                println!("Use the default port 80");
                port=80
            }
        };
        let root = root.to_string();
        WebServerConfig{
            root,
            listen_on:port
        }
    }
}