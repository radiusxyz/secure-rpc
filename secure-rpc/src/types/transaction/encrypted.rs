use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserEncryptedTransaction {
    encrypted_transaction: EncryptedTransaction,
    time_lock_puzzle: TimeLockPuzzle,
    nonce: Nonce,
}

impl AsRef<[u8]> for UserEncryptedTransaction {
    fn as_ref(&self) -> &[u8] {
        self.encrypted_transaction.as_ref()
    }
}

impl UserEncryptedTransaction {
    pub const ID: &'static str = stringify!(EncryptedTransaction);

    pub fn new(
        encrypted_transaction: EncryptedTransaction,
        time_lock_puzzle: TimeLockPuzzle,
        nonce: Nonce,
    ) -> Self {
        Self {
            encrypted_transaction,
            time_lock_puzzle,
            nonce,
        }
    }

    pub fn encrypted_transaction(&self) -> &EncryptedTransaction {
        &self.encrypted_transaction
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EncryptedTransaction(String);

impl AsRef<[u8]> for EncryptedTransaction {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl EncryptedTransaction {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().to_owned())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeLockPuzzle {
    t: u8,
    g: String,
    n: String,
}

impl TimeLockPuzzle {
    pub fn new(t: u8, g: impl AsRef<str>, n: impl AsRef<str>) -> Self {
        Self {
            t,
            g: g.as_ref().to_owned(),
            n: n.as_ref().to_owned(),
        }
    }
}
