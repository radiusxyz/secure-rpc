use serde::{Deserialize, Serialize};
use crate::rpc_primitives::*;
use tx_orderer::types::EthRawTransaction;
use secure_rpc_primitives::{AsyncTask, Context};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

impl EthSendRawTransaction {
    /// Get the first raw transaction from the list
    /// Since wallet gives transactions as a list even if it's only one transaction, 
    /// we need to get the first transaction from the list
    fn get_raw_tx(&self) -> Option<String> {
        self.0.first().cloned()
    }

    fn tx_hash(&self, raw_tx: String) -> Result<serde_json::Value, RpcError> {
        Ok(serde_json::to_value(EthRawTransaction(raw_tx).raw_transaction_hash().as_string()).map_err(|e| RpcError::from(e))?)
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
            let _ = context.async_task().send_tx(raw_tx.clone().into(), true).await.map_err(|e| RpcError::from(e))?;
            return Ok(self.tx_hash(raw_tx)?)
        }

        Ok(serde_json::Value::Null)
    }
}
