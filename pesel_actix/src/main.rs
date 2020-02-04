use actix_web::{get, error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/hi")]
async fn hi() -> impl Responder {
    format!("Hello world from actix-web")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(||
        App::new()
            .service(hi)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
