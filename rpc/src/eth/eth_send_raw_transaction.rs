use serde::{Deserialize, Serialize};
use crate::rpc_primitives::*;
use secure_rpc_primitives::{Context, ExternalRpcInterface, SecureRPCService};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

impl EthSendRawTransaction {
    fn get_raw_tx(&self) -> Option<String> {
        self.0.first().cloned()
    }
}

impl<C: Context> RpcParameter<C> for EthSendRawTransaction {
    type Response = serde_json::Value;

    fn method() -> &'static str {
        "eth_sendRawTransaction"
    }

    async fn handler(self, context: C) -> Result<serde_json::Value, RpcError> {
        if let Some(raw_tx) = self.get_raw_tx() {
            let (enc_key, session_id) = context.external_rpc_service().get_enc_key().await.map_err(|e| RpcError::from(e))?;
            let encrypted_tx = context.secure_rpc_service().encrypt_tx(session_id, &raw_tx, &enc_key).await.map_err(|e| RpcError::from(e))?;
            let res = context.external_rpc_service().forward_tx(context.rollup_id(), true, encrypted_tx).await.map_err(|e| RpcError::from(e))?;
            return Ok(res)
        }

        Ok(serde_json::Value::Null)
    }
}
