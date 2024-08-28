use actix_web::{get, HttpResponse, Responder};
use crate::models;

#[get("/api/ping")]
async fn ping_handler() -> impl Responder {
    const MESSAGE: &str = "ping";

    let response_json = models::GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}
