use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use redis::Client;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let route_table = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let route_table_ref = Arc::clone(&route_table);

    let make_svc = make_service_fn(move |_conn| {
        let route_table_ref = Arc::clone(&route_table_ref);
        async {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let route_table_ref = Arc::clone(&route_table_ref);
                async { bonsai_router(req, route_table_ref).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    let _schema_registry = thread::spawn(move || {
        let redis_client = Client::open("redis://127.0.0.1/").unwrap();
        let mut con = redis_client.get_connection().unwrap();
        let mut pubsub = con.as_pubsub();

        pubsub.subscribe("route").unwrap();

        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();
            println!("{}: {}", msg.get_channel_name(), payload);
            route_table
                .lock()
                .unwrap()
                .insert(format!("/{}", payload), payload.to_owned());
        }
    });

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn bonsai_router(
    req: Request<Body>,
    route_table: Arc<Mutex<HashMap<String, String>>>,
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
