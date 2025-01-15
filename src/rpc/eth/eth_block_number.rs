use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthBlockNumber(Value);

impl RpcParameter<AppState> for EthBlockNumber {
    type Response = Value;

    fn method() -> &'static str {
        "eth_blockNumber"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
