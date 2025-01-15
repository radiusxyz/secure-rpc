use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetCode(Value);

impl RpcParameter<AppState> for EthGetCode {
    type Response = Value;

    fn method() -> &'static str {
        "eth_getCode"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
