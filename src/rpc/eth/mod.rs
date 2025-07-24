mod eth_send_raw_transaction;

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

macro_rules! define_forwarding_rpc {
    ($name:ident, $method:expr) => {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct $name(Value);

        impl RpcParameter<AppState> for $name {
            type Response = Value;

            fn method() -> &'static str {
                $method
            }

            async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
                forward(Self::method(), self, context).await
            }
        }
    };
}

// Related to Eth rpc
define_forwarding_rpc!(EthBlockNumber, "eth_blockNumber");
define_forwarding_rpc!(EthChainId, "eth_chainId");
define_forwarding_rpc!(EthProtocolVersion, "eth_protocolVersion");
define_forwarding_rpc!(EthSyncing, "eth_syncing");
define_forwarding_rpc!(EthGasPrice, "eth_gasPrice");
define_forwarding_rpc!(EthMaxPriorityFeePerGas, "eth_maxPriorityFeePerGas");
define_forwarding_rpc!(EthFeeHistory, "eth_feeHistory");
define_forwarding_rpc!(EthGetBlockByHash, "eth_getBlockByHash");
define_forwarding_rpc!(EthGetBlockByNumber, "eth_getBlockByNumber");
define_forwarding_rpc!(
    EthGetBlockTransactionCountByHash,
    "eth_getBlockTransactionCountByHash"
);
define_forwarding_rpc!(
    EthGetBlockTransactionCountByNumber,
    "eth_getBlockTransactionCountByNumber"
);
define_forwarding_rpc!(
    EthGetUncleByBlockHashAndIndex,
    "eth_getUncleByBlockHashAndIndex"
);
define_forwarding_rpc!(
    EthGetUncleByBlockNumberAndIndex,
    "eth_getUncleByBlockNumberAndIndex"
);
define_forwarding_rpc!(EthGetUncleCountByBlockHash, "eth_getUncleCountByBlockHash");
define_forwarding_rpc!(
    EthGetUncleCountByBlockNumber,
    "eth_getUncleCountByBlockNumber"
);
define_forwarding_rpc!(EthGetTransactionByHash, "eth_getTransactionByHash");
define_forwarding_rpc!(EthGetRawTransactionByHash, "eth_getRawTransactionByHash");
define_forwarding_rpc!(
    EthGetTransactionByBlockHashAndIndex,
    "eth_getTransactionByBlockHashAndIndex"
);
define_forwarding_rpc!(
    EthRetRawTransactionByBlockHashAndIndex,
    "eth_retRawTransactionByBlockHashAndIndex"
);
define_forwarding_rpc!(
    EthGetTransactionByBlockNumberAndIndex,
    "eth_getTransactionByBlockNumberAndIndex"
);
define_forwarding_rpc!(
    EthRetRawTransactionByBlockNumberAndIndex,
    "eth_retRawTransactionByBlockNumberAndIndex"
);
define_forwarding_rpc!(EthGetTransactionReceipt, "eth_getTransactionReceipt");
define_forwarding_rpc!(EthGetBlockReceipts, "eth_getBlockReceipts");
define_forwarding_rpc!(EthEstimateGas, "eth_estimateGas");
define_forwarding_rpc!(EthGetBalance, "eth_getBalance");
define_forwarding_rpc!(EthGetCode, "eth_getCode");
define_forwarding_rpc!(EthGetTransactionCount, "eth_getTransactionCount");
define_forwarding_rpc!(EthGetStorageAt, "eth_getStorageAt");
define_forwarding_rpc!(EthCall, "eth_call");
define_forwarding_rpc!(EthCallMany, "eth_callMany");
define_forwarding_rpc!(EthCallBundle, "eth_callBundle");
define_forwarding_rpc!(EthCreateAccessList, "eth_createAccessList");
define_forwarding_rpc!(EthNewFilter, "eth_newFilter");
define_forwarding_rpc!(EthNewBlockFilter, "eth_newBlockFilter");
define_forwarding_rpc!(
    EthNewPendingTransactionFilter,
    "eth_newPendingTransactionFilter"
);
define_forwarding_rpc!(EthGetFilterLogs, "eth_getFilterLogs");
define_forwarding_rpc!(EthGetFilterChanges, "eth_getFilterChanges");
define_forwarding_rpc!(EthUninstallFilter, "eth_uninstallFilter");
define_forwarding_rpc!(EthGetLogs, "eth_getLogs");
define_forwarding_rpc!(EthSignTypedData, "eth_signTypedData");
define_forwarding_rpc!(EthGetProof, "eth_getProof");
define_forwarding_rpc!(EthMining, "eth_mining");
define_forwarding_rpc!(EthCoinbase, "eth_coinbase");
define_forwarding_rpc!(EthHashrate, "eth_hashrate");
define_forwarding_rpc!(EthSubmitHashrate, "eth_submitHashrate");
define_forwarding_rpc!(EthGetWork, "eth_getWork");
define_forwarding_rpc!(EthSubmitWork, "eth_submitWork");
define_forwarding_rpc!(EthSubscribe, "eth_subscribe");
define_forwarding_rpc!(EthUnsubscribe, "eth_unsubscribe");

// Related to Net rpc
define_forwarding_rpc!(NetVersion, "net_version");

// Related to Zkevm rpc
define_forwarding_rpc!(ZkevmBatchNumber, "zkevm_batchNumber");
define_forwarding_rpc!(
    ZkevmBatchNumberByBlockNumber,
    "zkevm_batchNumberByBlockNumber"
);
define_forwarding_rpc!(
    ZkevmConsolidatedBlockNumber,
    "zkevm_consolidatedBlockNumber"
);
define_forwarding_rpc!(ZkevmEstimateCounters, "zkevm_estimateCounters");
define_forwarding_rpc!(ZkevmGetBatchByNumber, "zkevm_getBatchByNumber");
define_forwarding_rpc!(
    ZkevmGetBatchCountersByNumber,
    "zkevm_getBatchCountersByNumber"
);
define_forwarding_rpc!(ZkevmGetBatchWitness, "zkevm_getBatchWitness");
define_forwarding_rpc!(ZkevmGetBlockRangeWitness, "zkevm_getBlockRangeWitness");
define_forwarding_rpc!(ZkevmGetExitRootTable, "zkevm_getExitRootTable");
define_forwarding_rpc!(ZkevmGetExitRootsByGER, "zkevm_getExitRootsByGER");
define_forwarding_rpc!(ZkevmGetForkById, "zkevm_getForkById");
define_forwarding_rpc!(ZkevmGetForkId, "zkevm_getForkId");
define_forwarding_rpc!(ZkevmGetForkIdByBatchNumber, "zkevm_getForkIdByBatchNumber");
define_forwarding_rpc!(ZkevmGetForks, "zkevm_getForks");
define_forwarding_rpc!(ZkevmGetFullBlockByHash, "zkevm_getFullBlockByHash");
define_forwarding_rpc!(ZkevmGetFullBlockByNumber, "zkevm_getFullBlockByNumber");
define_forwarding_rpc!(ZkevmGetL2BlockInfoTree, "zkevm_getL2BlockInfoTree");
define_forwarding_rpc!(
    ZkevmGetLatestDataStreamBlock,
    "zkevm_getLatestDataStreamBlock"
);
define_forwarding_rpc!(
    ZkevmGetLatestGlobalExitRoot,
    "zkevm_getLatestGlobalExitRoot"
);
define_forwarding_rpc!(ZkevmGetProverInput, "zkevm_getProverInput");
define_forwarding_rpc!(ZkevmGetRollupAddress, "zkevm_getRollupAddress");
define_forwarding_rpc!(
    ZkevmGetRollupManagerAddress,
    "zkevm_getRollupManagerAddress"
);
define_forwarding_rpc!(ZkevmGetVersionHistory, "zkevm_getVersionHistory");
define_forwarding_rpc!(ZkevmGetWitness, "zkevm_getWitness");
define_forwarding_rpc!(ZkevmIsBlockConsolidated, "zkevm_isBlockConsolidated");
define_forwarding_rpc!(ZkevmIsBlockVirtualized, "zkevm_isBlockVirtualized");
define_forwarding_rpc!(ZkevmVerifiedBatchNumber, "zkevm_verifiedBatchNumber");
define_forwarding_rpc!(ZkevmVirtualBatchNumber, "zkevm_virtualBatchNumber");
