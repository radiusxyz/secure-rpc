use crate::rpc::eth::{eth_get_block_by_number::EthBlock, prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetBlockByHash {
    pub block_hash: String,
    pub full_tx: bool,
}

impl RpcParameter<AppState> for EthGetBlockByHash {
    type Response = EthBlock<String>;

    fn method() -> &'static str {
        "eth_getBlockByHash"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(
            Self::method(),
            vec![
                serde_json::to_value(self.block_hash)?,
                serde_json::to_value(self.full_tx)?,
            ],
            context,
        )
        .await
    }
}
