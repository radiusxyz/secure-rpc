use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthCall {
    tx_data: EthTxData,
    _something: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EthTxData {
    to: String,
    data: String,
    from: Option<String>,
}

impl RpcParameter<AppState> for EthCall {
    type Response = String;

    fn method() -> &'static str {
        "eth_call"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), vec![self], context).await
    }
}
