use async_trait::async_trait;
// use radius_sequencer_sdk::liveness::types::Block;
use serde::{Deserialize, Serialize};

use crate::{
    impl_rollup_rpc_forwarder,
    rpc::{
        external::{forward_to_rpc_request, RollupRpcParameter},
        prelude::*,
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Block<TX> {
    #[serde(default, rename = "parentHash")]
    pub parent_hash: String,
    #[serde(default, rename = "sha3Uncles")]
    pub uncles_hash: String,
    #[serde(default, rename = "miner")]
    pub author: Option<String>,
    #[serde(default, rename = "stateRoot")]
    pub state_root: String,
    #[serde(default, rename = "transactionsRoot")]
    pub transactions_root: String,
    #[serde(default, rename = "receiptsRoot")]
    pub receipts_root: String,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<String>,
    #[serde(default)]
    pub difficulty: String,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<String>,
    pub size: Option<String>,
    pub number: Option<String>,
    #[serde(default, rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(default, rename = "gasUsed")]
    pub gas_used: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(default, rename = "extraData")]
    pub extra_data: String,
    #[serde(rename = "mixHash")]
    pub mix_hash: Option<String>,
    pub nonce: Option<String>,
    pub hash: Option<String>,
    #[serde(bound = "TX: Serialize + serde::de::DeserializeOwned", default)]
    pub transactions: Vec<TX>,
    #[serde(default)]
    pub uncles: Vec<String>,
    #[serde(rename = "baseFeePerGas", skip_serializing_if = "Option::is_none")]
    pub base_fee_per_gas: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "blobGasUsed"
    )]
    pub blob_gas_used: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "excessBlobGas"
    )]
    pub excess_blob_gas: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "withdrawalsRoot"
    )]
    pub withdrawals_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawals: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parentBeaconBlockRoot"
    )]
    pub parent_beacon_block_root: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetBlockByNumber {
    block_number: String,
    full_tx: bool,
}

impl_rollup_rpc_forwarder!(EthGetBlockByNumber, "eth_getBlockByNumber", Block<String>);
// impl_rollup_rpc_forwarder!(EthGetBlockByNumber, "eth_getBlockByNumber", Block);
