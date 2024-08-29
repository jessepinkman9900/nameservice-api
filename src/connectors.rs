pub mod base;
use base::BaseNameService;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use crate::models::{NameAvailableRequest, NameAvailableResponse};

#[derive(Serialize, Deserialize, Clone, EnumString, EnumIter)]
pub enum Chain {
    #[strum(serialize = "ethereum")]
    Ethereum,
    #[strum(serialize = "base")]
    Base,
    #[strum(serialize = "canto")]
    Canto,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Display)]
pub enum ChainNameService {
    BaseNameService,
    EthNameService,
    CantoNameService,
}

#[derive(Debug, EnumIter, Display)]
pub enum Operation {
    NameAvailable,
}

pub trait Connector<Req, Res> {
    async fn execute_operation(&self, _req: Req) -> Res;
}

pub async fn connector_core(
    _operation: Operation,
    req: NameAvailableRequest,
) -> NameAvailableResponse {
    let connector = get_name_available_connector(req.chain.clone());
    let response = connector.execute_operation(req).await;
    response
}

fn get_name_available_connector(
    _chain: Chain,
) -> impl Connector<NameAvailableRequest, NameAvailableResponse> {
    BaseNameService {
        base_url: format!("{}", "https://api.basename.app/v1"),
    }
}
