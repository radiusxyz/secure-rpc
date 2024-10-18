use serde_json::json;

use crate::{
    rpc::{prelude::*, LeakedStrGuard, RequestToSendEncryptedTransaction},
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

impl EthSendRawTransaction {
    pub const METHOD_NAME: &'static str = "eth_sendRawTransaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<String, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let raw_transaction_string = parameter.0.first().unwrap();

        let raw_transaction = EthRawTransaction(raw_transaction_string.clone());
        let raw_transaction_hash = raw_transaction.raw_transaction_hash();

        let raw_transaction_request_string = json!({
            "rollup_id": context.config().rollup_id(),
            "raw_transaction": {
                "type": "eth",
                "data": raw_transaction_string
            }
        })
        .to_string();

        let raw_transaction_static_str = LeakedStrGuard::new(raw_transaction_request_string);
        let raw_transaction_params = RpcParameter::new(Some(*raw_transaction_static_str));

        RequestToSendEncryptedTransaction::handler(raw_transaction_params, context).await?;

        Ok(raw_transaction_hash.as_string())
    }
}
