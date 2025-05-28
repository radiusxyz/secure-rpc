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

macro_rules! define_fowarding_rpc {
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
define_fowarding_rpc!(EthBlockNumber, "eth_blockNumber");
define_fowarding_rpc!(EthCall, "eth_call");
define_fowarding_rpc!(EthChainId, "eth_chainId");
define_fowarding_rpc!(EthEstimateGas, "eth_estimateGas");
define_fowarding_rpc!(EthFeeHistory, "eth_feeHistory");
define_fowarding_rpc!(EthGasPrice, "eth_gasPrice");
define_fowarding_rpc!(EthGetBalance, "eth_getBalance");
define_fowarding_rpc!(EthGetBlockByHash, "eth_getBlockByHash");
define_fowarding_rpc!(EthGetBlockByNumber, "eth_getBlockByNumber");
define_fowarding_rpc!(EthGetCode, "eth_getCode");
define_fowarding_rpc!(EthGetTransactionByHash, "eth_getTransactionByHash");
define_fowarding_rpc!(EthGetTransactionCount, "eth_getTransactionCount");
define_fowarding_rpc!(EthGetTransactionReceipt, "eth_getTransactionReceipt");
define_fowarding_rpc!(EthGetLogs, "eth_getLogs");

define_fowarding_rpc!(NetVersion, "net_version");
// define_eth_rpc!(EthGetCode, "eth_getCode");
// define_eth_rpc!(EthGasPrice, "eth_gasPrice");
// define_eth_rpc!(EthFeeHistory, "eth_feeHistory");
// define_eth_rpc!(EthGetBalance, "eth_getBalance");

// Related to Zkevm rpc
define_fowarding_rpc!(ZkevmGetLatestGlobalExitRoot, "zkevm_getLatestGlobalExitRoot");
define_fowarding_rpc!(ZkevmGetExitRootsByGER, "zkevm_getExitRootsByGER");

