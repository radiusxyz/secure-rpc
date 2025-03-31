use tx_orderer::types::{
    to_raw_tx, EncryptedTransaction, EthPlainData, EthRawTransaction, RawTransaction,
    SkdeEncryptedTransaction, TransactionData,
};

use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecryptTransaction {
    pub encrypted_transaction: EncryptedTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecryptTransactionResponse {
    pub raw_transaction: RawTransaction,
}

impl RpcParameter<AppState> for DecryptTransaction {
    type Response = DecryptTransactionResponse;

    fn method() -> &'static str {
        "decrypt_transaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        let decrypted_data = self.decrypt_transaction_data(&context).await?;
        self.decode_to_raw_transaction(&decrypted_data)
    }
}

impl DecryptTransaction {
    async fn decrypt_transaction_data(&self, context: &AppState) -> Result<String, RpcError> {
        match &self.encrypted_transaction {
            EncryptedTransaction::Skde(skde_tx) => {
                self.decrypt_skde_transaction(context, skde_tx).await
            }
        }
    }

    async fn decrypt_skde_transaction(
        &self,
        context: &AppState,
        skde_transaction: &SkdeEncryptedTransaction,
    ) -> Result<String, RpcError> {
        let dkg_client = context
            .distributed_key_generation_client()
            .ok_or(Error::DistributedKeyGenerationClientNotInitialized)?;

        let decryption_key = dkg_client
            .get_decryption_key(skde_transaction.key_id)
            .await?
            .decryption_key;

        let encrypted_data = self
            .encrypted_transaction
            .transaction_data()
            .encrypted_data()
            .clone()
            .into_inner();

        tracing::info!("Decrypting SKDE encrypted data");

        skde::delay_encryption::decrypt(context.skde_params(), &encrypted_data, &decryption_key)
            .map_err(Error::DecryptionError)
            .map_err(Into::into)
    }

    fn decode_to_raw_transaction(
        &self,
        decrypted_data: &str,
    ) -> Result<DecryptTransactionResponse, RpcError> {
        match self.encrypted_transaction.transaction_data() {
            TransactionData::Eth(eth_transaction_data) => {
                let eth_plain_data: EthPlainData =
                    serde_json::from_str(decrypted_data).map_err(|_| Error::DecodeFailed)?;

                let rollup_transaction = eth_transaction_data
                    .open_data
                    .convert_to_rollup_transaction(&eth_plain_data);

                let raw_transaction =
                    RawTransaction::from(EthRawTransaction::from(to_raw_tx(rollup_transaction)));

                Ok(DecryptTransactionResponse { raw_transaction })
            }
            _ => Err(Error::UnsupportedTransactionType.into()),
        }
    }
}
