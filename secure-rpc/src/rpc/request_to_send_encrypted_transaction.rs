use serde_json::json;

use super::EncryptTransaction;
use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestToSendEncryptedTransaction {
    rollup_id: String,
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: String,
    pub encrypted_transaction: EncryptedTransaction,
}

// TODO: Refactoring
impl RequestToSendEncryptedTransaction {
    pub const METHOD_NAME: &'static str = "send_encrypted_transaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        if !context.config().is_using_encryption() {
            return Err(Error::EncryptionNotEnabled.into());
        }

        let parameter = parameter.parse::<Self>()?;

        let raw_transaction_string = json!({
            "raw_transaction": parameter.raw_transaction
        })
        .to_string();

        println!("raw_transaction_string: {:?}", raw_transaction_string);
        let encrypt_transaction_static_str = LeakedStrGuard::new(raw_transaction_string);

        let encrypt_transaction_params = RpcParameter::new(Some(*encrypt_transaction_static_str));

        println!(
            "encrypt_transaction_params: {:?}",
            encrypt_transaction_params
        );

        // encrypt transaction
        let encrypt_transaction_response =
            EncryptTransaction::handler(encrypt_transaction_params, context.clone()).await?;

        print!(
            "encrypt_transaction_response: {:?}",
            encrypt_transaction_response
        );

        let send_encrypted_transaction = SendEncryptedTransaction {
            rollup_id: parameter.rollup_id,
            encrypted_transaction: encrypt_transaction_response.encrypted_transaction,
        };

        println!(
            "send_encrypted_transaction: {:?}",
            send_encrypted_transaction
        );

        context
            .sequencer_rpc_client()
            .rpc_client()
            .request(
                RequestToSendEncryptedTransaction::METHOD_NAME,
                send_encrypted_transaction,
            )
            .await
            .map_err(|error| error.into())
    }
}

use std::ops::Deref;

pub struct LeakedStrGuard {
    inner: &'static str,
}

impl LeakedStrGuard {
    pub fn new(s: String) -> Self {
        let boxed_str = s.into_boxed_str();
        let static_str = Box::leak(boxed_str);

        LeakedStrGuard { inner: static_str }
    }
}

impl Deref for LeakedStrGuard {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for LeakedStrGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.inner as *const str as *mut str);
        }
    }
}
