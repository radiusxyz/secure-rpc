use async_trait::async_trait;
// use radius_sequencer_sdk::liveness::types::Block;
use serde::{Deserialize, Serialize};

use super::eth_get_block_by_number::EthBlock;
use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{forward_to_array_rpc_request, prelude::*, ExternalRpcParameter},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetBlockByHash {
    block_hash: String,
    full_tx: bool,
}

impl_external_array_rpc_forwarder!(EthGetBlockByHash, "eth_getBlockByHash", EthBlock<String>);
