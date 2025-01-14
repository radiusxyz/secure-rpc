use radius_sdk::json_rpc::server::RpcError;
use serde_json::json;

use super::EncryptTransaction;
use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

impl RpcParameter<AppState> for SendEncryptedTransaction {
    type Response = OrderCommitment;

    fn method() -> &'static str {
        "send_encrypted_transaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        if !context.config().is_using_encryption() {
            return Err(Error::EncryptionNotEnabled.into());
        }

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
                self.rollup_id,
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
