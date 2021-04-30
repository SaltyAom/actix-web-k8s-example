use actix_web::{ get, Responder , web::ServiceConfig };

#[get("/")]
async fn landing() -> impl Responder {
    "Hello World"
}

#[get("/status")]
async fn status() -> impl Responder {
    "Working"
}

pub fn status_modules(config: &mut ServiceConfig) {
    config.service(landing).service(status);
}
