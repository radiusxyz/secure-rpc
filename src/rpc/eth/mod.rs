mod eth_block_number;
mod eth_call;
mod eth_chain_id;
mod eth_estimate_gas;
mod eth_fee_history;
mod eth_gas_price;
mod eth_get_balance;
mod eth_get_block_by_hash;
mod eth_get_block_by_number;
mod eth_get_code;
mod eth_get_transaction_by_hash;
mod eth_get_transaction_count;
mod eth_get_transaction_receipt;
mod eth_net_version;
mod eth_send_raw_transaction;

pub use eth_block_number::EthBlockNumber;
pub use eth_call::EthCall;
pub use eth_chain_id::EthChainId;
pub use eth_estimate_gas::EthEstimateGas;
pub use eth_fee_history::EthFeeHistory;
pub use eth_gas_price::EthGasPrice;
pub use eth_get_balance::EthGetBalance;
pub use eth_get_block_by_hash::EthGetBlockByHash;
pub use eth_get_block_by_number::EthGetBlockByNumber;
pub use eth_get_code::EthGetCode;
pub use eth_get_transaction_by_hash::EthGetTransactionByHash;
pub use eth_get_transaction_count::EthGetTransactionCount;
pub use eth_get_transaction_receipt::EthGetTransactionReceipt;
pub use eth_net_version::EthNetVersion;
pub use eth_send_raw_transaction::EthSendRawTransaction;

pub mod prelude {
    pub use radius_sdk::json_rpc::{
        client::{Id, RpcClient},
        server::{RpcError, RpcParameter},
    };
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub use serde_json::{Error, Value};

    pub use crate::state::AppState;
}

use prelude::*;

pub async fn forward<P, R>(method: &str, parameter: P, context: AppState) -> Result<R, RpcError>
where
    P: Serialize,
    R: DeserializeOwned,
{
    context
        .rpc_client()
        .request(
            context.config().rollup_rpc_url(),
            method,
            parameter,
            Id::Null,
        )
        .await
        .map_err(RpcError::from)
}
