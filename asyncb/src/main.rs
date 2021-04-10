#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::mem::size_of;

use futures_util::future::join;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

static INDEX1: &[u8] = b"The 1st service!";
static INDEX2: &[u8] = b"The 2nd service!";

async fn index1(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from(INDEX1)))
}

async fn index2(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from(INDEX2)))
}


fn main1() {
    let x = 5;
    let y = &x;
    
    println!("Does y == 5? {}", *y==5);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    main1();

    println!("Size of Char is {}", size_of::<char>());
    println!("Size of String is {}", size_of::<String>());
    println!("Size of Vec is {}", size_of::<Vec<i8>>());

    let x:(i32, f64, i32) = (1, 1.0, 5);
    let y = [[1,2,3], [2,3,4]];
    let mut s = String::from("hello world");
    s.push('1');
    println!("{:?}, {:?}, {}, {:?}", x, y, s, s);

    let addr1 = ([127, 0, 0, 1], 1337).into();
    let addr2 = ([127, 0, 0, 1], 1338).into();

    let srv1 = Server::bind(&addr1).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(index1))
    }));

    let srv2 = Server::bind(&addr2).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(index2))
    }));

    println!("Listening on http://{} and http://{}", addr1, addr2);

    let _ret = join(srv1, srv2).await;

    Ok(())
}
