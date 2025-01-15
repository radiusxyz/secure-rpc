use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetTransactionByHash(Value);

impl RpcParameter<AppState> for EthGetTransactionByHash {
    type Response = Value;

    fn method() -> &'static str {
        "eth_getTransactionByHash"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
