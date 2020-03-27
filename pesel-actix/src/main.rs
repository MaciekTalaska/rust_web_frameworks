use actix_web::{get, error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct PESELInfo {
    pesel_number: String,
    date_of_birth: String,
    gender: String,
    is_valid: bool,
    err: bool,
    err_message: String,
}

#[get("/hi")]
async fn hi() -> impl Responder {
    format!("Hello world from actix-web")
}

#[get("/pesel/{pesel}")]
async fn pesel_check(info: web::Path<String>) -> impl Responder {
    let result = pesel::pesel::PESEL::from_str(&info);
    match result {
        Ok(pesel) => {
            let pesel_json = PESELInfo {
                pesel_number: info.to_string(),
                date_of_birth: pesel.date_of_birth(),
                gender: pesel.gender_name(),
                is_valid: pesel.is_valid(),
                err: false,
                err_message: "".to_string(),
            };
            web::Json(pesel_json)
        }
        _ => {
            let pesel_json = PESELInfo {
                pesel_number: info.to_string(),
                date_of_birth: "invalid".to_string(),
                gender: "invalid".to_string(),
                is_valid: false,
                err: true,
                err_message: format!("{} is not a valid pesel number", info),
            };
            web::Json(pesel_json)
        }
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
    let pesel_json = PESELInfo {
        pesel_number: "whatever".to_string(),
        date_of_birth: generated_pesel.date_of_birth(),
        gender: generated_pesel.gender_name(),
        is_valid: generated_pesel.is_valid(),
        err: false,
        err_message: "".to_string(),
    };
    web::Json(pesel_json)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    println!("server started: http://localhost:8080/");
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
