use std::time::{SystemTime, UNIX_EPOCH};

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use tx_orderer::types::RawTransaction;

use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendRawTransaction {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

impl RpcParameter<AppState> for SendRawTransaction {
    type Response = serde_json::Value;

    fn method() -> &'static str {
        "send_raw_transaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .wrapping_add(self.rollup_id.len() as u128) as u64;

        let url = context
            .config()
            .tx_orderer_rpc_url_list()
            .choose(&mut StdRng::seed_from_u64(seed))
            .ok_or_else(|| {
                tracing::error!("No tx_orderer_rpc_url available in config");
                Error::EmptyTxOrdererRpcUrl
            })?;

        // 요청 전송
        match context
            .rpc_client()
            .request(url, Self::method(), self, Id::Null)
            .await
        {
            Ok(result) => Ok(result),
            Err(error) => {
                tracing::error!("Failed to send raw transaction: {:?}", error);
                Err(error.into())
            }
        }
    }
}
