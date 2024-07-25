mod encrypted;
mod raw;

pub use encrypted::*;
pub use raw::*;

use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UserTransaction {
    Raw(UserRawTransaction),
    Encrypted(UserEncryptedTransaction),
}

impl AsRef<[u8]> for UserTransaction {
    fn as_ref(&self) -> &[u8] {
        match self {
            UserTransaction::Raw(raw_transaction) => raw_transaction.as_ref(),
            UserTransaction::Encrypted(encrypted_transaction) => encrypted_transaction.as_ref(),
        }
    }
}

impl From<UserEncryptedTransaction> for UserTransaction {
    fn from(encrypted_transaction: UserEncryptedTransaction) -> Self {
        Self::Encrypted(encrypted_transaction)
    }
}

impl From<UserRawTransaction> for UserTransaction {
    fn from(raw_transaction: UserRawTransaction) -> Self {
        Self::Raw(raw_transaction)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Nonce(String);

impl Nonce {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().to_owned())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct OrderCommitment {
    pub rollup_block_number: u64,
    pub transaction_order: u64,
}

impl OrderCommitment {
    pub fn new(rollup_block_number: u64, transaction_order: u64) -> Self {
        Self {
            rollup_block_number,
            transaction_order,
        }
    }
}
