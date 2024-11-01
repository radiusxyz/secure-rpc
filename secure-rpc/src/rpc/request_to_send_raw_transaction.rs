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
    pub const METHOD_NAME: &'static str = "send_raw_transaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let order_commitment = context
            .sequencer_rpc_client()
            .send_raw_transaction(parameter.rollup_id, parameter.raw_transaction)
            .await?;

        Ok(order_commitment)
    }
}
