use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthCall {
    pub tx_data: EthTxData,
    pub _placeholder: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthTxData {
    pub to: String,
    pub data: String,
    pub from: Option<String>,
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
