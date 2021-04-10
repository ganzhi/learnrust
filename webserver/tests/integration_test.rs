extern crate reqwest;

mod common;

#[test]
fn it_adds_two() {
    let mut s = common::Setup::new();
    let client = reqwest::blocking::Client::new();
    let r = match reqwest::blocking::get("https://localhost:8181/hello.html"){
        Ok(res) => res,
        Err(e) => {
            println!("Test error: {}", e);
            return;
        }
    };
    println!("http response is: {}", r.text().unwrap());

    //println!("Response: {}", r.status());

    s.teardown();
}