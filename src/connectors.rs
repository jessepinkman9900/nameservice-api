pub mod base;
use crate::{
    models::{NameAvailableRequest, NameAvailableResponse},
    router::Router,
};
use base::BaseNameService;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
// use crate::models::{NameAvailableRequest, NameAvailableResponse};

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

#[derive(Clone)]
pub struct NameAvailableRouter {}

impl Router<NameAvailableRequest, NameAvailableResponse> for NameAvailableRouter {
    async fn route_operation(
        _operation: Operation,
        _chain: Chain,
        _req: NameAvailableRequest,
    ) -> NameAvailableResponse {
        let connector = Self::get_connector(_chain);
        let response = connector.execute_operation(_req).await;
        response
    }

    fn get_connector(_chain: Chain) -> impl Connector<NameAvailableRequest, NameAvailableResponse> {
        BaseNameService {
            base_url: format!("{}", "https://api.basename.app/v1"),
        }
    }
}
