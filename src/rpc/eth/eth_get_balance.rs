use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetBalance {
    address: String,
    block_number: String,
}

impl RpcParameter<AppState> for EthGetBalance {
    type Response = String;

    fn method() -> &'static str {
        "eth_getBalance"
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
