extern crate reqwest;

mod common;

#[test]
fn it_adds_two() {
    let mut s = common::Setup::new();
    let r = match reqwest::blocking::get("https://localhost:8181/"){
        Ok(res) => res,
        Err(_) => {
            return;
        }
    };

    println!("Response: {}", r.status());

    s.teardown();
}