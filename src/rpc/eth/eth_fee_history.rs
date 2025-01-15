use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthFeeHistory(Value);

impl RpcParameter<AppState> for EthFeeHistory {
    type Response = Value;

    fn method() -> &'static str {
        "eth_feeHistory"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
