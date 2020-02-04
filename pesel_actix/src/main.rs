use actix_web::{get, error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use std::str::FromStr;

#[get("/hi")]
async fn hi() -> impl Responder {
    format!("Hello world from actix-web")
}

#[get("/pesel/{pesel}")]
async fn pesel_check(info: web::Path<String>) -> impl Responder {
    let result = pesel::pesel::PESEL::from_str(&info);
    match result {
        Ok(pesel) => format!("{}", pesel),
        _ => format!("{} is not a valid pesel number", &info)
    }
}

#[get("/pesel/{year}/{month}/{day}/{gender}")]
async fn pesel_creation(info: web::Path<(u16, u8, u8, String)>) -> impl Responder {
    let gender = info.3.as_str();
    let biological_gender = match gender {
        "m" => pesel::pesel::PeselGender::Male,
        _ => pesel::pesel::PeselGender::Female,
    };
    let generated_pesel = pesel::pesel::PESEL::new(info.0, info.1, info.2, biological_gender);
    format!("{}", generated_pesel)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(||
        App::new()
            .service(hi)
            .service(pesel_check)
            .service( pesel_creation)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
