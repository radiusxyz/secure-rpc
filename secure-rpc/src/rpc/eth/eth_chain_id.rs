use async_trait::async_trait;

use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{forward_to_array_rpc_request, prelude::*, ExternalRpcParameter},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthChainId {}

impl_external_array_rpc_forwarder!(EthChainId, "eth_chainId", String);
