use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedTransactionType {
    Skde,
    NotSupport,
}

impl Default for EncryptedTransactionType {
    fn default() -> Self {
        Self::NotSupport
    }
}

impl From<String> for EncryptedTransactionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "skde" | "Skde" | "SKDE" => Self::Skde,
            _ => Self::NotSupport,
        }
    }
}
