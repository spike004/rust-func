use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Server, Body, Response, Request};
use crate::handlers;
use crate::handlers::validate_order:validate_order;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let result = validate_order();

    let response = format!("Result: {:?}", result);

    Ok(Response::new(Body::from(response)))
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn|{
        service_fn(handle_request)
    });

    let addr = SocketAddr::from(([17, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_svc);

    println!("Dev server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
