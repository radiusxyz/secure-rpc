use async_trait::async_trait;

use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{forward_to_array_rpc_request, prelude::*, ExternalRpcParameter},
};

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

impl_external_array_rpc_forwarder!(EthCall, "eth_call", String);
