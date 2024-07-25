use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserRawTransaction {
    raw_transaction: RawTransaction,
    nonce: Nonce,
}

impl AsRef<[u8]> for UserRawTransaction {
    fn as_ref(&self) -> &[u8] {
        self.raw_transaction.as_ref()
    }
}

impl UserRawTransaction {
    pub const ID: &'static str = stringify!(RawTransaction);

    pub fn new(raw_transaction: RawTransaction, nonce: Nonce) -> Self {
        Self {
            raw_transaction,
            nonce,
        }
    }

    pub fn raw_transaction(&self) -> &RawTransaction {
        &self.raw_transaction
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawTransaction(String);

impl AsRef<[u8]> for RawTransaction {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl RawTransaction {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().to_owned())
    }
}
