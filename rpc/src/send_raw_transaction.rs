use serde::{Deserialize, Serialize};
use tx_orderer::types::RawTransaction;
use radius_sdk::json_rpc::server::{RpcError, RpcParameter};
use secure_rpc_primitives::{Context, AsyncTask};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendRawTransaction {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

impl<C: Context> RpcParameter<C> for SendRawTransaction {
    type Response = serde_json::Value;

    fn method() -> &'static str {
        "send_raw_transaction"
    }

    async fn handler(self, context: C) -> Result<Self::Response, RpcError> {
        let res = context.async_task().send_tx(serde_json::to_vec(&self.raw_transaction)?, false).await.map_err(|e| RpcError::from(e))?;
        Ok(res)
    }
}
