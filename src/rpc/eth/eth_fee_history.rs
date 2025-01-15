use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthFeeHistory {}

impl RpcParameter<AppState> for EthFeeHistory {
    type Response = String;

    fn method() -> &'static str {
        "eth_feeHistory"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), Vec::<String>::default(), context).await
    }
}
