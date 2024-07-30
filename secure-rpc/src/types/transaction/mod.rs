mod encrypted_transaction;
mod raw_transaction;

pub use encrypted_transaction::*;
pub use raw_transaction::*;

use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Transaction {
    Raw(RawTransaction),
    Encrypted(EncryptedTransaction),
}

// impl AsRef<[u8]> for Transaction {
//     fn as_ref(&self) -> &[u8] {
//         match self {
//             Transaction::Raw(raw_transaction) => raw_transaction.as_ref(),
//             Transaction::Encrypted(encrypted_transaction) => encrypted_transaction.as_ref(),
//         }
//     }
// }

// impl From<EncryptedTransaction> for Transaction {
//     fn from(encrypted_transaction: EncryptedTransaction) -> Self {
//         Self::Encrypted(encrypted_transaction)
//     }
// }

// impl From<RawTransaction> for Transaction {
//     fn from(raw_transaction: RawTransaction) -> Self {
//         Self::Raw(raw_transaction)
//     }
// }

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Nonce(String);

// impl Nonce {
//     pub fn new(value: impl AsRef<str>) -> Self {
//         Self(value.as_ref().to_owned())
//     }
// }

// #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
// pub struct OrderCommitment {
//     pub rollup_block_number: u64,
//     pub transaction_order: u64,
// }

// impl OrderCommitment {
//     pub fn new(rollup_block_number: u64, transaction_order: u64) -> Self {
//         Self {
//             rollup_block_number,
//             transaction_order,
//         }
//     }
// }
