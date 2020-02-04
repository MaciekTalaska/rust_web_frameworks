use actix_web::{get, error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use std::str::FromStr;

#[get("/hi")]
async fn hi() -> impl Responder {
    format!("Hello world from actix-web")
}

#[get("/pesel/{pesel}")]
async fn pesel_check(info: web::Path<(String)>) -> impl Responder {
    let result = pesel::pesel::PESEL::from_str(&info);
    match result {
        Ok(pesel) => format!("{}", pesel),
        _ => format!("{} is not a valid pesel number", &info)
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(||
        App::new()
            .service(hi)
            .service(pesel_check)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
