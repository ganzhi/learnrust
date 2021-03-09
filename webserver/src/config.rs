use std::{fs::File, io::Read};
use toml::Value;
use toml::from_str;
pub struct WebServerConfig {
    pub root: String
}

impl WebServerConfig {
    pub fn new(path: &str) -> WebServerConfig {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        let _res = file.read_to_string(&mut contents).unwrap();
        let config: toml::Value = toml::from_str(&contents).unwrap();
        let root = config["root"].as_str().unwrap();
        let root = root.to_string();
        WebServerConfig{
            root
        }
    }
}