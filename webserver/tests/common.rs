use std::process::{Child, Command};

pub struct Setup {
    child:Child,
}

impl Setup {
    pub fn new() -> Setup {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--", "--config", "conf/config.toml"]);
        
        Setup{
            child:cmd.spawn().unwrap()
        }
    }

    pub fn teardown(&mut self) {
        self.child.kill();
    }
}