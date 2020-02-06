use warp::Filter;
use std::str::FromStr;

pub fn validate_pesel(pesel_number: &str) -> String {
    let p = pesel::pesel::PESEL::from_str(pesel_number);
    match p {
        Ok(pesel) => format!("{}",pesel),
        _ => format!("invalid pesel")
    }
}

#[tokio::main]
async fn main() {
    let hi = warp::path!("hi")
        .map(|| format!("Hello World from warp!"));

    let pesel_validator = warp::path!("pesel" / String)
        .map(|pesel: String| format!("argument was: {}", validate_pesel(pesel.as_str())));

    let routes = warp::get().and(hi.or(pesel_validator));
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
