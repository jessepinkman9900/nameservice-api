use serde::{Deserialize, Serialize};

use crate::configs::Chain;

#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub status: String,
    pub message: T,
}

// API Models

#[derive(Serialize, Deserialize)]
pub struct NameAvailableRequest {
    pub chain: Chain,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NameAvailableResponse {
    pub chain: Chain,
    pub name: String,
    pub available: bool,
}
