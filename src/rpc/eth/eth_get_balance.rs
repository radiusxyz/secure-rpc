use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetBalance(Value);

impl RpcParameter<AppState> for EthGetBalance {
    type Response = Value;

    fn method() -> &'static str {
        "eth_getBalance"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
