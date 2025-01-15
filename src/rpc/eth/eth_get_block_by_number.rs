use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetBlockByNumber(Value);

impl RpcParameter<AppState> for EthGetBlockByNumber {
    type Response = Value;

    fn method() -> &'static str {
        "eth_getBlockByNumber"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
