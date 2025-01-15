use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetTransactionCount {
    pub address: String,
    pub block_number: String,
}

impl RpcParameter<AppState> for EthGetTransactionCount {
    type Response = String;

    fn method() -> &'static str {
        "eth_getTransactionCount"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(
            Self::method(),
            vec![self.address, self.block_number],
            context,
        )
        .await
    }
}
