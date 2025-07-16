use skde::delay_encryption::{SkdeParams, encrypt};
use secure_rpc_primitives::SecureRPCService;
use async_trait::async_trait; 
use tx_orderer::types::{SkdeEncryptedTransaction, decode_rlp_transaction, 
    to_encrypt_data_string, EthOpenData, EncryptedData, 
    TransactionData, EthTransactionData
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SkdeSecureRPCService {
    /// The SKDE parameters
    skde_params: SkdeParams, 
}

impl SkdeSecureRPCService {
    pub fn new(skde_params: SkdeParams) -> Self {
        Self { skde_params }
    }

    fn decode_raw_tx(&self, raw_tx: &str) -> Result<(EthOpenData, String)> {
        // TODO: Refactor me!
        decode_rlp_transaction(raw_tx)
            .map(|tx| {
                (EthOpenData::from(tx.clone()), to_encrypt_data_string(&tx))
            }).map_err(|_| anyhow::anyhow!("RLP decode failed"))
    }

    fn encrypt_tx(&self, session_id: u64, raw_tx: &str, enc_key: &str) -> Result<SkdeEncryptedTransaction, SkdeSecureRPCServiceError> {
        let (tx_data , msg) = self.decode_raw_tx(raw_tx).map_err(|_| SkdeSecureRPCServiceError::RlpDecodeFailed)?;
        encrypt(&self.skde_params, &msg, enc_key, true)
            .map(|encrypted| {
                let encrypted_data = EncryptedData::from(encrypted);
                let transaction_data =
                    TransactionData::Eth(EthTransactionData::new(encrypted_data, tx_data));
                SkdeEncryptedTransaction::new(transaction_data, session_id)
            }).map_err(|_| SkdeSecureRPCServiceError::SkdeEncryptionFailed)
    }
}

#[async_trait]
impl SecureRPCService for SkdeSecureRPCService {
    type EncryptedTx = SkdeEncryptedTransaction;
    type Error = SkdeSecureRPCServiceError;

    async fn encrypt_tx(&self, session_id: u64, raw_tx: &str, enc_key: &str) -> Result<Self::EncryptedTx, Self::Error> {
        self.encrypt_tx(session_id, raw_tx, enc_key)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SkdeSecureRPCServiceError {
    #[error("RLP decode failed")]
    RlpDecodeFailed,
    #[error("SKDE encryption failed")]
    SkdeEncryptionFailed,
    #[error("SKDE parameters not initialized")]
    NotInitialized,
}

