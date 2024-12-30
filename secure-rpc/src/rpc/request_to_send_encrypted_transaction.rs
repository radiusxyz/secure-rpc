use radius_sdk::json_rpc::server::RpcError;
use serde_json::json;

use super::EncryptTransaction;
use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestToSendEncryptedTransaction {
    rollup_id: String,
    pub raw_transaction: RawTransaction,
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

        let encrypt_transaction_static_str = LeakedStrGuard::new(raw_transaction_string);
        let encrypt_transaction_params = RpcParameter::new(Some(*encrypt_transaction_static_str));

        println!(
            "encrypt_transaction_params: {:?}",
            encrypt_transaction_params
        );

        // encrypt transaction
        let encrypt_transaction_response =
            EncryptTransaction::handler(encrypt_transaction_params, context.clone()).await?;

        match context
            .sequencer_rpc_client()
            .send_encrypted_transaction(
                parameter.rollup_id,
                encrypt_transaction_response.encrypted_transaction,
            )
            .await
        {
            Ok(order_commitment) => {
                tracing::info!("Order commitment: {:?}", order_commitment);
                Ok(order_commitment)
            }
            Err(e) => {
                tracing::error!("Failed to send encrypted transaction: {:?}", e);
                Err(e.into())
            }
        }
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
