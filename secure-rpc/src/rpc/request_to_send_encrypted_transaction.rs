use json_rpc::Params;
use serde_json::{json, value::RawValue};

use super::EncryptTransaction;
use crate::{
    rpc::{encrypt_transaction::EncryptTransactionResponse, prelude::*},
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestToSendEncryptedTransaction {
    rollup_id: RollupId,
    raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: RollupId,
    pub encrypted_transaction: EncryptedTransaction,
    pub time_lock_puzzle: TimeLockPuzzle,
}

impl RequestToSendEncryptedTransaction {
    pub const METHOD_NAME: &'static str = "request_to_send_encrypted_transaction";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        // TODO(jaemin): impl SendEncryptedTransaction or import from Sequencer
        const SEND_ENCRYPTED_TRANSACTION: &str = "send_encrypted_transaction";

        let parameter = parameter.parse::<Self>()?;

        let encrypt_transaction = EncryptTransaction {
            raw_transaction: parameter.raw_transaction,
        };

        let encrypt_transaction_response = context
            .secure_rpc_client()
            .rpc_client()
            .request::<_, EncryptTransactionResponse>(
                EncryptTransaction::METHOD_NAME,
                encrypt_transaction,
            )
            .await
            .map_err(|error| RpcError::from(error))?;

        let send_encrypted_transaction = SendEncryptedTransaction {
            rollup_id: parameter.rollup_id,
            encrypted_transaction: encrypt_transaction_response.encrypted_transaction,
            time_lock_puzzle: encrypt_transaction_response.time_lock_puzzle,
        };

        println!(
            "jaemin - send_encrypted_transaction: {:?}",
            send_encrypted_transaction
        );

        // TODO(jaemin): merge with sequencer
        // context
        //     .sequencer_rpc_client()
        //     .rpc_client()
        //     .request(SEND_ENCRYPTED_TRANSACTION, send_encrypted_transaction)
        //     .await
        //     .map_err(|error| error.into())
        Ok(())
    }
}
