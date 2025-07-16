use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use tx_orderer::types::{EncryptedTransaction, RawTransaction};
use radius_sdk::json_rpc::server::{RpcError, RpcParameter};
use secure_rpc_primitives::{Context, SecureRPCService, ExternalRpcInterface};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTx {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTxParam<T> {
    pub rollup_id: String,
    pub encrypted_tx: T,
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

        tracing::info!("{}: {:?}", Self::method(), self.raw_transaction);

        let raw_tx: String = serde_json::from_str(&serde_json::to_string(&self.raw_transaction)?)?;
        let (session_id, enc_key) = context.dkg_client_service().get_enc_key().await.map_err(|e| RpcError::from(e))?;
        let encrypted_tx = context.secure_rpc_service().encrypt_tx(session_id, &raw_tx, &enc_key).await.map_err(|e| RpcError::from(e))?;
        
        let parameter = SendEncryptedTxParam {
            rollup_id: self.rollup_id.clone(),
            encrypted_tx,
        };
        
        let response = context.rpc_client_service().request(Self::method(), parameter).await?;

        let url = self.select_url(&context, &parameter)?;

        // Send to tx_orderer
        self.send_to_tx_orderer(context, &url, parameter).await
    }
}

impl SendEncryptedTx {

    fn select_url(
        &self,
        context: &AppState,
        parameter: &SendEncryptedTransactionRequest,
    ) -> Result<String, Error> {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .wrapping_add(parameter.rollup_id.len() as u128) as u64;

        let url = context
            .config()
            .tx_orderer_rpc_url_list()
            .choose(&mut StdRng::seed_from_u64(seed))
            .ok_or(Error::EmptyTxOrdererRpcUrl)?;

        Ok(url.to_string())
    }

    async fn send_to_tx_orderer(
        &self,
        context: AppState,
        url: &str,
        parameter: SendEncryptedTransactionRequest,
    ) -> Result<serde_json::Value, RpcError> {
        match context
            .rpc_client()
            .request(url, Self::method(), parameter, Id::Null)
            .await
        {
            Ok(result) => {
                tracing::debug!("Encrypted transaction sent successfully");
                Ok(result)
            }
            Err(error) => {
                tracing::error!(?error, "Failed to send encrypted transaction");
                Err(error.into())
            }
        }
    }
}
