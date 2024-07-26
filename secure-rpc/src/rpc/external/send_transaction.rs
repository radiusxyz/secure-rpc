use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendTransaction {
    pub rollup_id: u32,
    pub transaction: UserTransaction,
}

impl SendTransaction {
    pub const METHOD_NAME: &'static str = stringify!(SendTransaction);

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        context
            .sequencer_rpc_client()
            .rpc_client()
            .request(SendTransaction::METHOD_NAME, parameter)
            .await
            .map_err(|error| error.into())
    }
}
