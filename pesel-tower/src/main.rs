#[macro_use]
extern crate tower_web;
extern crate tokio;
extern crate pesel;

use tower_web::ServiceBuilder;
use tokio::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Hello;

#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

//impl_web! {
//    impl Hello {
//        #[get("/hi")]
//        #[content_type("json")]
//        fn hi(&self) -> Result<HelloResponse, ()> {
//            Ok(HelloResponse{message: "hello world from tower-web" })
//        }
//    }
//}

#[derive(Clone, Debug)]
struct PeselResource;

#[derive(Response)]
struct PeselInfoResponse {
    message: &'static str,
}

impl_web! {
    impl PeselResource {
        #[get("/pesel/:pesel")]
        fn pesel_check(&self, pesel: String) -> Result<String, ()> {
            let new_pesel = pesel::pesel::PESEL::from_str(&pesel);
            match new_pesel {
                Ok(p) => Ok( format!("{}", p)),
                _ => Ok( format!("not a proper PESEL number"))
            }
        }

        #[get("/pesel/:year/:month/:day/:gender")]
        fn pesel_creation(&self, year: u16, month: u8, day: u8, gender: String) -> Result<String, ()> {
            let biological_gender = match gender.as_str() {
                "m" => pesel::pesel::PeselGender::Male,
                _ => pesel::pesel::PeselGender::Female
            };
            let pesel = pesel::pesel::PESEL::new(year, month, day, biological_gender);
            Ok( format!("{}", pesel))
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");

    ServiceBuilder::new()
//        .resource(Hello)
        .resource(PeselResource)
        .run(&addr)
        .unwrap();
}
