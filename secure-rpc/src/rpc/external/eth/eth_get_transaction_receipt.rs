use async_trait::async_trait;

use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{
        external::{forward_to_array_rpc_request, ExternalRpcParameter},
        prelude::*,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetTransactionReceipt {}

impl_external_array_rpc_forwarder!(
    EthGetTransactionReceipt,
    "eth_getTransactionReceipt",
    String
);
