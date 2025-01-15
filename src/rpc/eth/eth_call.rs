use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthCall(Value);

impl RpcParameter<AppState> for EthCall {
    type Response = Value;

    fn method() -> &'static str {
        "eth_call"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
