use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

async fn bonsai_router(req: Request<Body>, count: i32) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/foo") => {
            *response.body_mut() = Body::from(format!("FOO {}", count));
        }
        (&Method::GET, "/bar") => {
            *response.body_mut() = Body::from("BAR");
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let counter = Arc::new(Mutex::new(0));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(move |_conn| {
        let counter = Arc::clone(&counter);
        async {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let mut count = counter.lock().unwrap();
                *count += 1;
                println!("{:}", count);
                let count_copy = *count;
                async move { bonsai_router(req, count_copy).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
