use async_trait::async_trait;

use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{
        {forward_to_array_rpc_request, ExternalRpcParameter},
        prelude::*,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthTransactionForEstimateGas {
    pub from: String,
    pub to: String,
    #[serde(default, rename = "gasPrice")]
    pub gas_price: String,
    pub value: String,
    pub data: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthEstimateGas {
    pub tx: EthTransactionForEstimateGas,
}

impl_external_array_rpc_forwarder!(EthEstimateGas, "eth_estimateGas", String);
