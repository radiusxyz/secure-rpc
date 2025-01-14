use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthBlockNumber {}

impl RpcParameter<AppState> for EthBlockNumber {
    type Response = String;

    fn method() -> &'static str {
        "eth_blockNumber"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), Vec::<String>::default(), context).await
    }
}
