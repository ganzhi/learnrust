use tokio::fs::File;
use tokio::fs;

use tokio_util::codec::{BytesCodec, FramedRead};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};

static INDEX: &str = "src/send_file_index.html";
static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:1337".parse().unwrap();

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response_examples)) });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    println!("method {} uri {}", req.method(), req.uri().path());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(INDEX).await,
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be be found
            simple_file_send("this_file_should_not_exist.html").await
        },
        (&Method::GET, s) => {
            let filename = &s[1..];
            
            if let Ok(m) = fs::metadata(filename).await {
                if m.is_dir() {
                    let mut buf = String::new();
                    if let Ok(mut r) = fs::read_dir(filename).await {
                        loop {
                            let n = r.next_entry().await;
                            match n {
                                Ok(Some(e)) => {
                                    buf.push_str(e.path().to_str().unwrap());
                                    buf.push('\n');
                                },
                                Ok(None) => {break;},
                                Err(_) => {}
                            };
                        }

                    }
                    let response  = Response::builder()
                    .status(StatusCode::OK)
                    .body(buf.into())
                    .unwrap();
                    Ok(response)
                }
                else {
                    simple_file_send(filename).await
                }
            }
            else {
                Ok(not_found())
            }
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}
