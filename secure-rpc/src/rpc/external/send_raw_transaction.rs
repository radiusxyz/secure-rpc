use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendRawTransaction {
    pub rollup_id: u32,
    pub raw_transaction: UserRawTransaction,
}

impl SendRawTransaction {
    pub const METHOD_NAME: &'static str = stringify!(SendRawTransaction);

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        context
            .sequencer_rpc_client()
            .rpc_client()
            .request(SendRawTransaction::METHOD_NAME, parameter)
            .await
            .map_err(|error| error.into())
    }
}
