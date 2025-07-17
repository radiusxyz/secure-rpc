use crate::rpc_primitives::*;
use secure_rpc_primitives::{Context, ExternalRpcInterface};
mod eth_send_raw_transaction;
pub use eth_send_raw_transaction::EthSendRawTransaction;

macro_rules! eth_rpc {
    [$(($name: ident, $method: expr)),*] => {
        $(
            #[derive(Clone, Debug, Deserialize, Serialize)]
            pub struct $name(serde_json::Value);

            impl<C: Context> RpcParameter<C> for $name {
                type Response = serde_json::Value;

                fn method() -> &'static str {
                    $method
                }

                async fn handler(self, context: C) -> Result<Self::Response, RpcError> {
                    context.external_rpc_service().forward_rpc_request(<Self as RpcParameter<C>>::method(), self.0).await.map_err(|e| RpcError::from(e))
                }
            }
        )*
    }
}

eth_rpc! [
    (EthBlockNumber, "eth_blockNumber"),
    (EthCall, "eth_call"),
    (EthChainId, "eth_chainId"),
    (EthEstimateGas, "eth_estimateGas"),
    (EthFeeHistory, "eth_feeHistory"),
    (EthGasPrice, "eth_gasPrice"),
    (EthGetBalance, "eth_getBalance"),
    (EthGetBlockByHash, "eth_getBlockByHash"),
    (EthGetBlockByNumber, "eth_getBlockByNumber"),
    (EthGetCode, "eth_getCode"),
    (EthGetTransactionByHash, "eth_getTransactionByHash"),
    (EthGetTransactionCount, "eth_getTransactionCount"),
    (EthGetTransactionReceipt, "eth_getTransactionReceipt"),
    (EthGetLogs, "eth_getLogs"),
    (EthGetStorageAt, "eth_getStorageAt"),
    (NetVersion, "net_version"),
    (ZkevmGetLatestGlobalExitRoot, "zkevm_getLatestGlobalExitRoot"),
    (ZkevmGetExitRootsByGER, "zkevm_getExitRootsByGER")
];

