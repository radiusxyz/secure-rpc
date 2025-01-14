use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

#[derive(Debug, Serialize)]
struct RawTransactionRequest<'a> {
    rollup_id: &'a str,
    raw_transaction: RawTransactionRequestData<'a>,
}

#[derive(Debug, Serialize)]
struct RawTransactionRequestData<'a> {
    #[serde(rename = "type")]
    transaction_type: &'a str,
    data: &'a str,
}

impl RpcParameter<AppState> for EthSendRawTransaction {
    type Response = String;

    fn method() -> &'static str {
        "eth_sendRawTransaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        if self.0.is_empty() {
            return Err(Error::EmptyRawTransaction.into());
        }

        let raw_transaction = RawTransactionRequest {
            rollup_id: context.config().rollup_id(),
            raw_transaction: RawTransactionRequestData {
                transaction_type: "eth",
                data: self.0.get(0).unwrap(),
            },
        };

        let _response: String = context
            .rpc_client()
            .request(
                context.config().sequencer_rpc_url(),
                "send_raw_transaction",
                raw_transaction,
                Id::Null,
            )
            .await?;

        Ok(String::from(""))
    }
}
