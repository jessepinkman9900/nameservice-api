use actix_web::middleware::Logger;
use actix_web::web::resource;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::ptr::read;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/api/ping")]
async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "ping";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(health_check_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
