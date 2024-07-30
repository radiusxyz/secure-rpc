use crate::types::prelude::*;

// TODO(jaemin): import types from sequencer feature
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthTransaction {
    open_data: OpenData,
    encrypted_transaction: EncryptedData,
}

// TODO: stompesi
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OpenData {
    pub raw_tx_hash: String,
}
