//! Done
use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthNetVersion {}

impl RpcParameter<AppState> for EthNetVersion {
    type Response = String;

    fn method() -> &'static str {
        "net_version"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), Vec::<String>::default(), context).await
    }
}
