extern crate futures;
extern crate hyper;

#[macro_use]
extern crate serde_json;

use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};

// This is here because we use map_error
use futures::Future;

fn main() {
    let router = || {
        service_fn_ok(|req| match (req.method(), req.uri().path()) {
            (&Method::GET, "/ping") => {
                Response::new(Body::from(json!({"message":"ping pong"}).to_string()))
            }
            (_, _) => {
                let mut res = Response::new(Body::from("not found"));
                *res.status_mut() = StatusCode::NOT_FOUND;
                res
            }
        })
    };

    // Setup and run the server
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Server::bind(&addr).serve(router);
    hyper::rt::run(server.map_err(|e| {
        eprintln!("server error: {}", e);
    }));
}
