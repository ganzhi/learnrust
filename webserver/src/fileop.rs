use std::fs;
use std::fmt::{format};
use std::path::Path;
use indoc::*;
use log::{info, warn, error, debug};

pub fn dir_to_html(path: &Path, url:String) -> String{
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
    let rd = fs::read_dir(path).unwrap();
    let path = Path::new(&url);
    for entry in rd {
        match entry {
            Ok(e) => {
                let p = e.file_name();
                let path_str = p.to_str().unwrap();
                let href = format!("<a href={}>", path.join(path_str).to_str().unwrap());
                dir_content.push_str(&href);
                dir_content.push_str(path_str);
                dir_content.push_str("</a><br/>\n");
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
    return dir_content;
}