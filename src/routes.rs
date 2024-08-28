use actix_web::{get, HttpResponse, Responder};
use strum::IntoEnumIterator;
use crate::models;
use crate::connectors::NameServices;

#[get("/ping")]
async fn ping_handler() -> impl Responder {
    const MESSAGE: &str = "ping";

    HttpResponse::Ok().json(MESSAGE)
}


#[get("/nameservice/supported")]
async fn nameservice_supported_handler() -> impl Responder {
    let namespace_strings: Vec<String> = NameServices::iter()
        .map(|ns| ns.to_string())
        .collect();

    HttpResponse::Ok().json(namespace_strings)
}