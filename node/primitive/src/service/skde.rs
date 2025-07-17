use skde::delay_encryption::{SkdeParams, encrypt};
use secure_rpc_primitives::SecureRpcService;
use async_trait::async_trait; 
use tx_orderer::types::{SkdeEncryptedTransaction, decode_rlp_transaction, 
    to_encrypt_data_string, EthOpenData, EncryptedData, 
    TransactionData, EthTransactionData
};
use anyhow::Result;

use crate::BlockchainService;

#[derive(Debug, Clone)]
pub struct SkdeSecureRpcService {
    /// The blockchain service that provides the SKDE parameters
    blockchain_service: BlockchainService,
    /// The SKDE parameters
    skde_params: Option<SkdeParams>, 
}

impl SkdeSecureRpcService {
    /// Create a new instance
    pub fn new(url: &str, contract_address: &str) -> Self {
        let blockchain_service = BlockchainService::new(url, contract_address);
        Self { blockchain_service, skde_params: None }
    }

    /// Get the SKDE parameters from the blockchain service if not already initialized
    pub async fn with_skde_params(&mut self) -> Result<(), SecureRpcServiceError> {
        if self.skde_params.is_none() {
            let res = self.blockchain_service.get_trusted_setup().await.map_err(|_| SecureRpcServiceError::SkdeParamsUnavailable)?;
            self.skde_params = Some(res);
        }
        Ok(())
    }

    fn decode_raw_tx(&self, raw_tx: &str) -> Result<(EthOpenData, String)> {
        // TODO: Refactor me!
        decode_rlp_transaction(raw_tx)
            .map(|tx| {
                (EthOpenData::from(tx.clone()), to_encrypt_data_string(&tx))
            }).map_err(|_| anyhow::anyhow!("RLP decode failed"))
    }

    fn encrypt_tx(&self, session_id: u64, raw_tx: &str, enc_key: &str) -> Result<SkdeEncryptedTransaction, SecureRpcServiceError> {
        let (tx_data , msg) = self.decode_raw_tx(raw_tx).map_err(|_| SecureRpcServiceError::RlpDecodeFailed)?;
        let skde_params = self.skde_params.as_ref().ok_or(SecureRpcServiceError::NotInitialized)?;
        encrypt(skde_params, &msg, enc_key, true)
            .map(|encrypted| {
                let encrypted_data = EncryptedData::from(encrypted);
                let transaction_data =
                    TransactionData::Eth(EthTransactionData::new(encrypted_data, tx_data));
                SkdeEncryptedTransaction::new(transaction_data, session_id)
            }).map_err(|_| SecureRpcServiceError::SkdeEncryptionFailed)
    }
}

#[async_trait]
impl SecureRpcService for SkdeSecureRpcService {
    type EncryptedTx = SkdeEncryptedTransaction;
    type Error = SecureRpcServiceError;

    async fn encrypt_tx(&self, session_id: u64, raw_tx: &str, enc_key: &str) -> Result<Self::EncryptedTx, Self::Error> {
        self.encrypt_tx(session_id, raw_tx, enc_key)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SecureRpcServiceError {
    #[error("RLP decode failed")]
    RlpDecodeFailed,
    #[error("SKDE encryption failed")]
    SkdeEncryptionFailed,
    #[error("SKDE parameters not initialized")]
    NotInitialized,
    #[error("SKDE parameters unavailable")]
    SkdeParamsUnavailable,
}

