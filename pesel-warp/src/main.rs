use warp::Filter;

#[tokio::main]
async fn main() {
    let hi = warp::path!("hi")
        .map(|| format!("Hello World from warp!"));

    warp::serve(hi)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
