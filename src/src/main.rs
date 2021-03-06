use actix_web::{ HttpServer, App, get, Responder };

#[get("/")]
async fn hello() -> impl Responder {
    "Hi"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
