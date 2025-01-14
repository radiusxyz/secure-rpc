use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGasPrice {}

impl RpcParameter<AppState> for EthGasPrice {
    type Response = String;

    fn method() -> &'static str {
        "eth_gasPrice"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), Vec::<String>::default(), context).await
    }
}
