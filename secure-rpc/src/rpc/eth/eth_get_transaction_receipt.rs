use async_trait::async_trait;

use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{forward_to_array_rpc_request, prelude::*, ExternalRpcParameter},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthGetTransactionReceipt {
    transaction_hash: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionReceipt {
    #[serde(default, rename = "cumulativeGasUsed")]
    cumulative_gas_used: String,
    #[serde(default, rename = "logsBloom")]
    logs_bloom: String,
    logs: Vec<String>,
    status: String,
    #[serde(default, rename = "transactionHash")]
    transaction_hash: String,
    #[serde(default, rename = "transactionIndex")]
    transaction_index: String,
    #[serde(default, rename = "blockHash")]
    block_hash: String,
    #[serde(default, rename = "blockNumber")]
    block_number: String,
    #[serde(default, rename = "gasUsed")]
    gas_used: String,
    from: String,
    to: String,
    #[serde(default, rename = "contractAddress")]
    contract_address: Option<String>,
    r#type: String,
    #[serde(default, rename = "effectiveGasPrice")]
    effective_gas_price: String,
}

impl_external_array_rpc_forwarder!(
    EthGetTransactionReceipt,
    "eth_getTransactionReceipt",
    TransactionReceipt
);
