use crate::connectors;
use crate::connectors::{connector_core, Chain, ChainNameService};
use crate::models::{NameAvailableRequest, NameAvailableResponse};
use actix_web::{get, web, HttpResponse, Responder};
use futures::future::join_all;
use itertools::Itertools;
use serde::Deserialize;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[get("/ping")]
async fn ping_handler() -> impl Responder {
    const MESSAGE: &str = "ping";

    HttpResponse::Ok().json(MESSAGE)
}

#[get("/nameservice/supported")]
async fn nameservice_supported_handler() -> impl Responder {
    let namespace_strings: Vec<String> =
        ChainNameService::iter().map(|ns| ns.to_string()).collect();

    HttpResponse::Ok().json(namespace_strings)
}

#[derive(Deserialize)]
struct QueryChains {
    chains: String,
}

#[get("/nameservice/available/{name}")]
async fn nameservice_name_avaiable_handler(
    path: web::Path<String>,
    query: web::Query<QueryChains>,
) -> impl Responder {
    let name = path.into_inner();
    let chains: Vec<Chain> = query
        .chains
        .split(",")
        .map(|c| c.to_lowercase())
        .unique()
        .map(|c| Chain::from_str(&c).unwrap())
        .collect();

    let responses: Vec<NameAvailableResponse> = join_all(chains.iter().map(|c| {
        let req = NameAvailableRequest {
            chain: c.clone(),
            name: name.clone(),
        };
        connector_core(connectors::Operation::NameAvailable, req)
    }))
    .await;
    HttpResponse::Ok().json(responses)
}
