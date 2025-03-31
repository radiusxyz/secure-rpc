use skde::delay_encryption::SkdeParams;
use tx_orderer::types::{
    decode_rlp_transaction, to_encrypt_data_string, EncryptedData, EncryptedTransaction,
    EthOpenData, EthTransactionData, RawTransaction, SkdeEncryptedTransaction, TransactionData,
};

use crate::{rpc::prelude::*, types::transaction::EncryptedTransactionType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EncryptTransaction {
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EncryptTransactionResponse {
    pub encrypted_transaction: EncryptedTransaction,
}

impl RpcParameter<AppState> for EncryptTransaction {
    type Response = EncryptTransactionResponse;

    fn method() -> &'static str {
        "encrypt_transaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        let raw_transaction_string =
            serde_json::to_string(&self.raw_transaction).map_err(|_| Error::SerializationError)?;

        match context.config().encrypted_transaction_type() {
            EncryptedTransactionType::Skde => {
                let skde_params = context.skde_params();
                let dkg_client = context
                    .distributed_key_generation_client()
                    .ok_or(Error::DistributedKeyGenerationClientNotInitialized)?;

                let key_response = dkg_client.get_latest_encryption_key().await?;
                let skde_encrypted = skde_encrypt_transaction(
                    skde_params,
                    &raw_transaction_string,
                    &key_response.key_id,
                    &key_response.encryption_key,
                )?;

                Ok(EncryptTransactionResponse {
                    encrypted_transaction: EncryptedTransaction::Skde(skde_encrypted),
                })
            }
            _ => Err(Error::UnsupportedEncryptionType.into()),
        }
    }
}

pub fn get_open_and_to_encrypt_data(raw_tx: &str) -> Result<(EthOpenData, String), Error> {
    decode_rlp_transaction(raw_tx)
        .map(|decoded_transaction| {
            let to_encrypt_data = to_encrypt_data_string(&decoded_transaction);
            let open_data = EthOpenData::from(decoded_transaction);
            (open_data, to_encrypt_data)
        })
        .map_err(|err| {
            tracing::error!("RLP decode failed: {:?}", err);
            Error::DecodeFailed
        })
}

pub fn skde_encrypt_transaction(
    skde_params: &SkdeParams,
    raw_transaction: &str,
    key_id: &u64,
    encryption_key: &str,
) -> Result<SkdeEncryptedTransaction, Error> {
    let (open_data, to_encrypt_data) = get_open_and_to_encrypt_data(raw_transaction)?;

    skde::delay_encryption::encrypt(skde_params, &to_encrypt_data, encryption_key)
        .map(|encrypted| {
            let encrypted_data = EncryptedData::from(encrypted);
            let transaction_data =
                TransactionData::Eth(EthTransactionData::new(encrypted_data, open_data));
            SkdeEncryptedTransaction::new(transaction_data, *key_id)
        })
        .map_err(|error| {
            tracing::error!("SKDE encryption failed: {:?}", error);
            Error::EncryptionError(error)
        })
}
