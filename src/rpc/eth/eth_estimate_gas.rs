use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthEstimateGas {
    pub tx: EthTransactionForEstimateGas,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthTransactionForEstimateGas {
    pub from: String,
    pub to: String,
    #[serde(default, rename = "gasPrice")]
    pub gas_price: String,
    pub value: String,
    pub data: String,
}

impl RpcParameter<AppState> for EthEstimateGas {
    type Response = String;

    fn method() -> &'static str {
        "eth_estimateGas"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), vec![self.tx], context).await
    }
}
