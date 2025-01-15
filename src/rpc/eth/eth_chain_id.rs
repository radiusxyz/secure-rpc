use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthChainId(Value);

impl RpcParameter<AppState> for EthChainId {
    type Response = Value;

    fn method() -> &'static str {
        "eth_chainId"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
