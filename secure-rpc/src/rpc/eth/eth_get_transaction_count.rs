use async_trait::async_trait;

use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{forward_to_array_rpc_request, prelude::*, ExternalRpcParameter},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetTransactionCount {
    address: String,
    block_number: String,
}

impl_external_array_rpc_forwarder!(EthGetTransactionCount, "eth_getTransactionCount", String);
