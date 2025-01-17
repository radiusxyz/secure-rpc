use rand::seq::SliceRandom;
use serde_json::Value;

use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

#[derive(Debug, Serialize)]
struct RawTransactionRequest<'a> {
    pub rollup_id: &'a str,
    pub raw_transaction: RawTransactionRequestData<'a>,
}

#[derive(Debug, Serialize)]
struct RawTransactionRequestData<'a> {
    #[serde(rename = "type")]
    transaction_type: &'a str,
    data: &'a str,
}

impl RpcParameter<AppState> for EthSendRawTransaction {
    type Response = Value;

    fn method() -> &'static str {
        "eth_sendRawTransaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        if self.0.is_empty() {
            return Err(Error::EmptyRawTransaction.into());
        }

        let raw_transaction_string = self.0.get(0).unwrap();
        let eth_raw_transaction = EthRawTransaction(raw_transaction_string.clone());
        let raw_transaction_hash = eth_raw_transaction.raw_transaction_hash();

        let parameter = RawTransactionRequest {
            rollup_id: context.config().rollup_id(),
            raw_transaction: RawTransactionRequestData {
                transaction_type: "eth",
                data: raw_transaction_string,
            },
        };

        let order_commitment: OrderCommitment = context
            .rpc_client()
            .request(
                context
                    .config()
                    .sequencer_rpc_url_list()
                    .choose(&mut rand::thread_rng())
                    .ok_or(Error::EmptySequencerRpcUrl)?,
                "send_raw_transaction",
                parameter,
                Id::Null,
            )
            .await?;

        println!("Order commitment: {:?}", order_commitment);

        Ok(serde_json::to_value(raw_transaction_hash.as_string())?)
    }
}
