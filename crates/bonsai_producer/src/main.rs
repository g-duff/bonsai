use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Method, Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn bonsai_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/foo") => {
            *response.body_mut() = Body::from("FOO");
        },
        (&Method::GET, "/bar") => {
            *response.body_mut() = Body::from("BAR");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(bonsai_router))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
