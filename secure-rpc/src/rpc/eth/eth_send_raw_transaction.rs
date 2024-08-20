use serde_json::json;

use crate::{
    rpc::{prelude::*, LeakedStrGuard, RequestToSendRawTransaction},
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

impl EthSendRawTransaction {
    pub const METHOD_NAME: &'static str = "eth_sendRawTransaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<OrderCommitment, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let raw_transaction_string = parameter
            .0
            .first()
            .and_then(|raw_transaction| serde_json::to_string(raw_transaction).ok())
            .ok_or(Error::DecodeFailed)?;

        let raw_transaction_request_string = json!({
            "rollup_id": context.config().rollup_id(),
            "raw_transaction": {
                "Eth": serde_json::from_str::<String>(&raw_transaction_string)?
            }
        })
        .to_string();

        let raw_transaction_static_str = LeakedStrGuard::new(raw_transaction_request_string);
        let raw_transaction_params = RpcParameter::new(Some(*raw_transaction_static_str));

        RequestToSendRawTransaction::handler(raw_transaction_params, context).await
    }
}
