use skde::delay_encryption::{SkdeParams, encrypt};
use secure_rpc_primitives::SecureRpcService;
use async_trait::async_trait; 
use tx_orderer::types::{SkdeEncryptedTransaction, decode_rlp_transaction, 
    to_encrypt_data_string, EthOpenData, EncryptedData, 
    TransactionData, EthTransactionData
};
use anyhow::Result;
use crate::DkgContract;

impl From<DkgContract::TrustedSetupParams> for SkdeParams {
    fn from(params: DkgContract::TrustedSetupParams) -> Self {
        Self {
            n: params.n,
            g: params.g,
            t: params.t,
            h: params.h,
            max_sequencer_number: params.max_sequencer_number,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkdeSecureRpcService(SkdeParams);

impl SkdeSecureRpcService {
    /// Create a new instance
    pub fn new(skde_params: SkdeParams) -> Self { Self(skde_params) }

    fn update_skde_params(&mut self, skde_params: SkdeParams) {
        self.0 = skde_params;
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
        encrypt(&self.0, &msg, enc_key, true)
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
    type TrustedSetup = SkdeParams;
    type EncryptedTx = SkdeEncryptedTransaction;
    type Error = SecureRpcServiceError;

    async fn update_trusted_setup(&mut self, trusted_setup: Self::TrustedSetup) {
        self.update_skde_params(trusted_setup.into());
    }

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

