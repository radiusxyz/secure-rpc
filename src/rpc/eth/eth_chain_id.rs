use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthChainId {}

impl RpcParameter<AppState> for EthChainId {
    type Response = String;

    fn method() -> &'static str {
        "eth_chainId"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), Vec::<String>::default(), context).await
    }
}
