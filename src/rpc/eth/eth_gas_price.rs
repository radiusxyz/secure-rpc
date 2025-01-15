use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGasPrice(Value);

impl RpcParameter<AppState> for EthGasPrice {
    type Response = Value;

    fn method() -> &'static str {
        "eth_gasPrice"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
