use rand::seq::SliceRandom;

use crate::rpc::{prelude::*, EncryptTransaction};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SendEncryptedTransactionRequest {
    rollup_id: String,
    encrypted_transaction: EncryptedTransaction,
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

        tracing::info!("encrypt_transaction_params: {:?}", self.raw_transaction);
        let encrypt_transaction_request = EncryptTransaction {
            raw_transaction: self.raw_transaction,
        };
        let encrypt_transaction_response =
            encrypt_transaction_request.handler(context.clone()).await?;

        let parameter = SendEncryptedTransactionRequest {
            rollup_id: self.rollup_id,
            encrypted_transaction: encrypt_transaction_response.encrypted_transaction,
        };

        match context
            .rpc_client()
            .request(
                context
                    .config()
                    .sequencer_rpc_url_list()
                    .choose(&mut rand::thread_rng())
                    .ok_or(Error::EmptySequencerRpcUrl)?,
                Self::method(),
                parameter,
                Id::Null,
            )
            .await
        {
            Ok(order_commitment) => {
                tracing::info!("Order commitment: {:?}", order_commitment);
                Ok(order_commitment)
            }
            Err(error) => {
                tracing::error!("Failed to send encrypted transaction: {:?}", error);
                Err(error.into())
            }
        }
    }
}
