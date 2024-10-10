use serde_json::json;

use crate::{
    rpc::{
        prelude::*, LeakedStrGuard, RequestToSendEncryptedTransaction, RequestToSendRawTransaction,
    },
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

        println!("parameter: {:?}", parameter);

        let raw_transaction_string = parameter.0.first().unwrap();

        let raw_transaction = EthRawTransaction(raw_transaction_string.clone());
        let raw_transaction_hash = raw_transaction.raw_transaction_hash();

        println!("raw_transaction_hash: {:?}", raw_transaction_hash);

        let raw_transaction_request_string = json!({
            "rollup_id": context.config().rollup_id(),
            "raw_transaction": {
                "type": "eth",
                "data": raw_transaction_string
            }
        })
        .to_string();

        println!(
            "raw_transaction_request_string: {:?}",
            raw_transaction_request_string
        );

        let raw_transaction_static_str = LeakedStrGuard::new(raw_transaction_request_string);
        let raw_transaction_params = RpcParameter::new(Some(*raw_transaction_static_str));

        // let order_commitment =
        //     RequestToSendRawTransaction::handler(raw_transaction_params, context).await?;

        let order_commitment =
            RequestToSendEncryptedTransaction::handler(raw_transaction_params, context).await?;

        println!("stompesi - order_commitment: {:?}", order_commitment);
        println!(
            "stompesi - raw_transaction_hash - 2: {:?}",
            raw_transaction_hash.clone().as_string()
        );

        Ok(raw_transaction_hash.as_string())
    }
}
