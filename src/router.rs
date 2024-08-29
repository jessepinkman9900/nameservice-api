use crate::configs::{Chain, Operation};
use crate::connectors::{base, Connector};
use crate::models::{NameAvailableRequest, NameAvailableResponse};
use base::BaseNameService;

pub trait Router<Req, Res> {
    async fn route_operation(_operation: Operation, _chain: Chain, _req: Req) -> Res;
    fn get_connector(_chain: Chain) -> impl Connector<Req, Res>;
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
