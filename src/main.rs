extern crate futures;
extern crate hyper;

use futures::{future, Future, Stream};
use hyper::header::ContentLength;
use hyper::server::{Client, Http, Request, Response, Server, Service};

struct HelloWorld;

const PHRASE: &'static str = "Hello, World!";

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE),
        ))
    }
}

fn main() {
    let addr = "127.0.0.1:8888".parse().unwrap();

    hyper::rt::run(future::lazy(move || {
        let client = Client::new();

        let server = Server::bind(&addr)
            .serve(HelloWorld)
            .map_err(|e| eprintln!("server error: {}", e));
        println!("Listening on http://{}", addr);

        server
    }));
}
