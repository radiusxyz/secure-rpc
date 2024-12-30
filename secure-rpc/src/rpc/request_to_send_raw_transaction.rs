use radius_sdk::json_rpc::server::RpcError;

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
    pub const METHOD_NAME: &'static str = "request_to_send_raw_transaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        match context
            .sequencer_rpc_client()
            .send_raw_transaction(parameter.rollup_id, parameter.raw_transaction)
            .await
        {
            Ok(order_commitment) => {
                tracing::info!("Order commitment: {:?}", order_commitment);
                Ok(order_commitment)
            }
            Err(e) => {
                tracing::error!("Failed to send raw transaction: {:?}", e);
                Err(e.into())
            }
        }
    }
}
