use warp::Filter;
use std::str::FromStr;

use pesel::pesel::PESEL;
use pesel::pesel::PeselGender;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct PESELInfo {
    pesel_number: String,
    date_of_birth: String,
    gender: String,
    is_valid: bool,
}

pub fn validate_pesel(pesel_number: &str) ->  warp::reply::Json {
    let p = PESEL::from_str(pesel_number);
    match p {
        Ok(pesel) => {
            let pesel_json = PESELInfo {
                pesel_number: pesel_number.to_string(),
                gender: pesel.gender_name(),
                date_of_birth: pesel.date_of_birth(),
                is_valid: pesel.is_valid()
            };

            warp::reply::json(&pesel_json)
        }

        _ => warp::reply::json(&"invalid pesel")
    }
}

pub fn generate_pesel(year: u16, month: u8, day: u8, gender: String) -> warp::reply::Json {
    let biological_gender = match gender.as_str() {
        "m" => PeselGender::Male,
        _ => PeselGender::Female
    };

    let pesel = PESEL::new(year, month, day, biological_gender);
    let pesel_json = PESELInfo {
        pesel_number: "whatever".to_string(), // this will be updated after new version of PESEL is published to crates.io
        gender: pesel.gender_name(),
        date_of_birth: pesel.date_of_birth(),
        is_valid: pesel.is_valid(),
    };
    warp::reply::json(&pesel_json)
}

#[tokio::main]
async fn main() {
    let hi = warp::path!("hi")
        .map(|| {
            warp::http::Response::builder()
                .header("content-type", "application/json; charset=utf-8");
                let msg = "Hello World from Warp!";
                warp::reply::json(&msg)});

    let pesel_validator = warp::path!("pesel" / String)
        .map(|pesel: String| {
           validate_pesel(pesel.as_str())
        });

    let pesel_generator = warp::path!("pesel" / u16 / u8 / u8 / String)
        .map(|year, month, day, gender| generate_pesel(year, month, day, gender ));

    let routes = warp::get().
        and(hi
            .or(pesel_validator)
            .or(pesel_generator)
        );

    println!("warp listening on: 127.0.0.1:8080");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
