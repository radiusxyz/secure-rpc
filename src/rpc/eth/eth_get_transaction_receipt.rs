use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetTransactionReceipt(Value);

impl RpcParameter<AppState> for EthGetTransactionReceipt {
    type Response = Value;

    fn method() -> &'static str {
        "eth_getTransactionReceipt"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
