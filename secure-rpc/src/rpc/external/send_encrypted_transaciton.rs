use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendEncryptedTransaction {
    pub rollup_id: u32,
    pub encrypted_transaction: UserEncryptedTransaction,
}

impl SendEncryptedTransaction {
    pub const METHOD_NAME: &'static str = stringify!(Send);

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        context
            .secure_rpc_client()
            .rpc_client()
            .request(SendEncryptedTransaction::METHOD_NAME, parameter)
            .await
            .map_err(|error| error.into())
    }
}
