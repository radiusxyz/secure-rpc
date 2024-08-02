use serde_json::value::RawValue;

use super::EncryptTransaction;
use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestToSendEncryptedTransaction {
    pub encrypt_transaction: EncryptTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: u32,
    pub encrypted_transaction: EncryptedTransaction,
    pub time_lock_puzzle: TimeLockPuzzle,
}

impl RequestToSendEncryptedTransaction {
    pub const METHOD_NAME: &'static str = stringify!(RequestToSendEncryptedTransaction);

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let json_string = serde_json::to_string(&parameter.encrypt_transaction)?;
        let raw_value = RawValue::from_string(json_string)?;
        // TODO(jaemin): change leak
        let encrypt_transaction_params = Params::new(Some(Box::leak(raw_value).get()));

        // encrypt transaction
        let encrypt_transaction_response =
            EncryptTransaction::handler(encrypt_transaction_params, context.clone()).await?;

        let send_encrypted_transaction = SendEncryptedTransaction {
            rollup_id: parameter.encrypt_transaction.rollup_id,
            encrypted_transaction: encrypt_transaction_response.encrypted_transaction,
            time_lock_puzzle: encrypt_transaction_response.time_lock_puzzle,
        };

        // TODO(jaemin): impl SendEncryptedTransaction or import from Sequencer
        const SEND_ENCRYPTED_TRANSACTION: &str = "SendEncryptedTransaction";

        context
            .sequencer_rpc_client()
            .rpc_client()
            .request(SEND_ENCRYPTED_TRANSACTION, send_encrypted_transaction)
            .await
            .map_err(|error| error.into())
    }
}
