use std::fs;
use std::path::Path;
use indoc::*;
use log::{info, warn, error, debug};

pub fn dir_to_html(path: &Path) -> String{
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
    return dir_content;
}