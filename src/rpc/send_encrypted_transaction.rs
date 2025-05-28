use std::time::{SystemTime, UNIX_EPOCH};

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use tx_orderer::types::{EncryptedTransaction, RawTransaction};

use crate::{
    rpc::{prelude::*, EncryptTransaction},
    types::transaction::EncryptedTransactionType,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransactionRequest {
    pub rollup_id: String,
    pub encrypted_transaction: EncryptedTransaction,
}

impl RpcParameter<AppState> for SendEncryptedTransaction {
    type Response = serde_json::Value;

    fn method() -> &'static str {
        "send_encrypted_transaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        let is_surpported =
            *context.config().encrypted_transaction_type() != EncryptedTransactionType::NotSupport;
        if !is_surpported {
            return Err(Error::EncryptionNotEnabled.into());
        }

        tracing::info!("encrypt_transaction_params: {:?}", self.raw_transaction);

        // Encrypt the transaction
        let encrypted_transaction = self.encrypt_transaction(context.clone()).await?;

        let parameter = SendEncryptedTransactionRequest {
            rollup_id: self.rollup_id.clone(),
            encrypted_transaction,
        };

        let url = self.select_url(&context, &parameter)?;

        // Send to tx_orderer
        self.send_to_tx_orderer(context, &url, parameter).await
    }
}

impl SendEncryptedTransaction {
    async fn encrypt_transaction(
        &self,
        context: AppState,
    ) -> Result<EncryptedTransaction, RpcError> {
        let encrypted_transaction = EncryptTransaction {
            raw_transaction: self.raw_transaction.clone(),
        }
        .handler(context)
        .await?
        .encrypted_transaction;

        Ok(encrypted_transaction)
    }

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
