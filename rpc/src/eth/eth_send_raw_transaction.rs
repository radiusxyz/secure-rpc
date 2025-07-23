use serde::{Deserialize, Serialize};
use crate::rpc_primitives::*;
use tx_orderer::types::EthRawTransaction;
use secure_rpc_primitives::{Context, ExternalRpcInterface, SecureRpcService};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

impl EthSendRawTransaction {
    /// Get the first raw transaction from the list
    /// Since wallet gives transactions as a list even if it's only one transaction, 
    /// we need to get the first transaction from the list
    fn get_raw_tx(&self) -> Option<String> {
        self.0.first().cloned()
    }
}

impl<C: Context> RpcParameter<C> for EthSendRawTransaction {
    type Response = serde_json::Value;

    fn method() -> &'static str {
        "eth_sendRawTransaction"
    }

    /// Handles the `eth_sendRawTransaction` RPC method if there is a transaction
    async fn handler(self, context: C) -> Result<serde_json::Value, RpcError> {
        if let Some(raw_tx) = self.get_raw_tx() {
            let (enc_key, session_id) = context.external_rpc_service().get_enc_key().await.map_err(|e| RpcError::from(e))?;
            let encrypted_tx = context.secure_rpc_service().encrypt_tx(session_id, &raw_tx, &enc_key).await.map_err(|e| RpcError::from(e))?;
            let _ = context.external_rpc_service().forward_tx(context.rollup_id(), true, encrypted_tx).await.map_err(|e| RpcError::from(e))?;
            return Ok(serde_json::to_value(EthRawTransaction(raw_tx).raw_transaction_hash()).map_err(|e| RpcError::from(e))?)
        }

        Ok(serde_json::Value::Null)
    }
}
