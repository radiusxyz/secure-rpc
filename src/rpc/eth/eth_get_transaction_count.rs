use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetTransactionCount(Value);

impl RpcParameter<AppState> for EthGetTransactionCount {
    type Response = Value;

    fn method() -> &'static str {
        "eth_getTransactionCount"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
