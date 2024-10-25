use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestToSendRawTransaction {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendRawTransaction {
    pub rollup_id: String,
    raw_transaction: RawTransaction,
}

impl RequestToSendRawTransaction {
    pub const METHOD_NAME: &'static str = "send_raw_transaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let send_raw_transaction = SendRawTransaction {
            rollup_id: parameter.rollup_id,
            raw_transaction: parameter.raw_transaction,
        };

        tracing::info!("Send raw transaction: {:?}", send_raw_transaction);

        context
            .sequencer_rpc_client()
            .rpc_client()
            .request(
                RequestToSendRawTransaction::METHOD_NAME,
                send_raw_transaction,
            )
            .await
            .map_err(|error| error.into())
    }
}
