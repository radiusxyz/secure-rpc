use serde::{Deserialize, Serialize};
use tx_orderer::types::RawTransaction;
use radius_sdk::json_rpc::server::{RpcError, RpcParameter};
use secure_rpc_primitives::{Context, SecureRPCService, ExternalRpcInterface};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTx {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

impl<C: Context> RpcParameter<C> for SendEncryptedTx {
    type Response = serde_json::Value;

    fn method() -> &'static str {
        "send_encrypted_transaction"
    }

    async fn handler(self, context: C) -> Result<Self::Response, RpcError> {
        if !context.is_encrypt_enabled() {
            return Ok(serde_json::Value::Null)
        }

        tracing::info!("{}", <Self as RpcParameter<C>>::method());

        let raw_tx: String = serde_json::from_str(&serde_json::to_string(&self.raw_transaction)?)?;
        let (enc_key, session_id) = context.external_rpc_service().get_enc_key().await.map_err(|e| RpcError::from(e))?;
        let encrypted_tx = context.secure_rpc_service().encrypt_tx(session_id, &raw_tx, &enc_key).await.map_err(|e| RpcError::from(e))?;
        let res = context.external_rpc_service().forward_tx(&self.rollup_id, true, encrypted_tx).await.map_err(|e| RpcError::from(e))?;

        Ok(res)
    }
}