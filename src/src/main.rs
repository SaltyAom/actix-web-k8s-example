mod modules;

use actix_web::{ HttpServer, App, middleware::Compress };
use modules::status::controllers::status_modules;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .configure(status_modules)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
