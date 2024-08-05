use json_rpc::Params;
use serde_json::{json, value::RawValue};

use super::EncryptTransaction;
use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestToSendEncryptedTransaction {
    rollup_id: RollupId,
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: RollupId,
    pub encrypted_transaction: EncryptedTransaction,
    pub time_lock_puzzle: TimeLockPuzzle,
}

impl RequestToSendEncryptedTransaction {
    pub const METHOD_NAME: &'static str = "request_to_send_encrypted_transaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        // TODO(jaemin): impl SendEncryptedTransaction or import from Sequencer
        const SEND_ENCRYPTED_TRANSACTION: &str = "send_encrypted_transaction";

        let parameter = parameter.parse::<Self>()?;

        let raw_tx = serde_json::to_string(&parameter.raw_transaction).unwrap();

        let raw_transaction_string = json!({
            "raw_transaction": raw_tx
        })
        .to_string();

        let raw_value = RawValue::from_string(raw_transaction_string)?;
        let encrypt_transaction_params = Params::new(Some(Box::leak(raw_value).get()));

        // encrypt transaction
        let encrypt_transaction_response =
            EncryptTransaction::handler(encrypt_transaction_params, context.clone()).await?;

        let send_encrypted_transaction = SendEncryptedTransaction {
            rollup_id: parameter.rollup_id,
            encrypted_transaction: encrypt_transaction_response.encrypted_transaction,
            time_lock_puzzle: encrypt_transaction_response.time_lock_puzzle,
        };

        context
            .sequencer_rpc_client()
            .rpc_client()
            .request(SEND_ENCRYPTED_TRANSACTION, send_encrypted_transaction)
            .await
            .map_err(|error| error.into())
    }
}
