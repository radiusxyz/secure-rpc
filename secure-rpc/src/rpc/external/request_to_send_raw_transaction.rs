use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestToSendRawTransaction {
    rollup_id: u32,
    raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendRawTransaction {
    pub rollup_id: u32,
    raw_transaction: RawTransaction,
}

impl RequestToSendRawTransaction {
    pub const METHOD_NAME: &'static str = stringify!(RequestToSendRawTransaction);

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let send_raw_transaction = SendRawTransaction {
            rollup_id: parameter.rollup_id,
            raw_transaction: parameter.raw_transaction,
        };

        // TODO(jaemin): impl SendRawTransaction or import from Sequencer
        const SEND_RAW_TRANSACTION: &str = "SendRawTransaction";

        context
            .sequencer_rpc_client()
            .rpc_client()
            .request(SEND_RAW_TRANSACTION, send_raw_transaction)
            .await
            .map_err(|error| error.into())
    }
}
