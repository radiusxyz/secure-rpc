use serde::{Deserialize, Serialize};
use tx_orderer::types::RawTransaction;
use radius_sdk::json_rpc::server::{RpcError, RpcParameter};
use secure_rpc_primitives::{Context, ExternalRpcInterface};

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
        let res = context.external_rpc_service().forward_tx(&self.rollup_id, false, self.raw_transaction.clone()).await.map_err(|e| RpcError::from(e))?;
        Ok(res)
    }
}
