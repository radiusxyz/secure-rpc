use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetBlockByHash(Value);

impl RpcParameter<AppState> for EthGetBlockByHash {
    type Response = Value;

    fn method() -> &'static str {
        "eth_getBlockByHash"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
