use warp::Filter;
use std::str::FromStr;

pub fn validate_pesel(pesel_number: &str) -> String {
    let p = pesel::pesel::PESEL::from_str(pesel_number);
    match p {
        Ok(pesel) => format!("{}",pesel),
        _ => format!("invalid pesel")
    }
}

pub fn generate_pesel(year: u16, month: u8, day: u8, gender: String) -> pesel::pesel::PESEL {
    let biological_gender = match gender.as_str() {
        "m" => pesel::pesel::PeselGender::Male,
        _ => pesel::pesel::PeselGender::Female
    };

    pesel::pesel::PESEL::new(year, month, day, biological_gender)
}

#[tokio::main]
async fn main() {
    let hi = warp::path!("hi")
        .map(|| format!("Hello World from warp!"));

    let pesel_validator = warp::path!("pesel" / String)
        .map(|pesel: String| format!("{}", validate_pesel(pesel.as_str())));

    let pesel_generator = warp::path!("pesel" / u16 / u8 / u8 / String)
        .map(|year, month, day, gender| format!("{}", generate_pesel(year, month, day, gender )));

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
