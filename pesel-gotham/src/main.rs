use gotham::router::builder::build_simple_router;
use gotham::state::State;
use gotham::router::Router;
use gotham::router::builder::*;

fn get_hi_handler(state: State) -> (State, String) {
    (state, "Hello World from Gotham".to_string())
}


fn router() -> Router {
    build_simple_router(|route| {
        route.get("/hi").to(get_hi_handler);
    })
}

fn main() {
    let address = "127.0.0.1:8080";
    println!("Listening for requestst at {}", address);
    gotham::start(address, router());
}
