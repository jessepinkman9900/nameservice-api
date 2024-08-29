use crate::connectors::{Chain, Connector, Operation};

pub trait Router<Req, Res> {
    async fn route_operation(_operation: Operation, _chain: Chain, _req: Req) -> Res;
    fn get_connector(_chain: Chain) -> impl Connector<Req, Res>;
}
