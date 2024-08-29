use crate::models::{NameAvailableRequest, NameAvailableResponse};

use super::Connector;

#[derive(Clone)]
pub struct BaseNameService {
    pub base_url: String,
}

impl Connector<NameAvailableRequest, NameAvailableResponse> for BaseNameService {
    async fn execute_operation(&self, _req: NameAvailableRequest) -> NameAvailableResponse {
        let url: String = format!(
            "{}/registration/{}/is-name-available",
            self.base_url, _req.name
        );
        let body = reqwest::get(url).await;

        NameAvailableResponse {
            chain: _req.chain,
            name: _req.name,
            available: body.unwrap().text().await.unwrap().clone().parse().unwrap(),
        }
    }
}
