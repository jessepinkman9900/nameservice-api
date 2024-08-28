mod routes;
mod models;
mod connectors;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            // middleware
            .wrap(Logger::default())
            // routes
            .service(routes::ping_handler)
            .service(routes::nameservice_supported_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
