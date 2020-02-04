#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tokio::prelude::*;

#[derive(Clone, Debug)]
struct Hello;

#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

impl_web! {
    impl Hello {
        #[get("/hi")]
        #[content_type("json")]
        fn hi(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse{message: "hello world from tower-web" })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");

    ServiceBuilder::new()
        .resource(Hello)
        .run(&addr)
        .unwrap();
}
