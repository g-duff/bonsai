use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let route_table = Arc::new(Mutex::new(HashMap::<&str, String>::new()));
    let route_table_ref = Arc::clone(&route_table);

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(move |_conn| {
        let route_table_ref = Arc::clone(&route_table_ref);
        async {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
            let route_table_ref = Arc::clone(&route_table_ref);
                async { bonsai_router(req, route_table_ref).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    route_table.lock().unwrap().insert("/foo", String::from("foo"));
    route_table.lock().unwrap().insert("/bar", String::from("bar"));

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn bonsai_router(
    req: Request<Body>,
    route_table: Arc<Mutex<HashMap::<&str, String>>>,
) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match route_table.lock().unwrap().get(req.uri().path()) {
        Some(x) => {
            *response.body_mut() = Body::from(x.clone());
        }
        None => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

