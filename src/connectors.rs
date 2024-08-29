pub mod base;

pub trait Connector<Req, Res> {
    async fn execute_operation(&self, _req: Req) -> Res;
}
