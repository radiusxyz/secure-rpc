use secure_rpc_primitives::Context;
use radius_sdk::json_rpc::server::RpcServer;
use secure_rpc_service::*;

macro_rules! register_rpc_method {
    ($server: expr, [$($rpc_type: ty),*]) => {
        $server$(
            .register_rpc_method::<$rpc_type>()
            .expect(format!("Failed to register RPC method: {}", stringify!($rpc_type)).as_str())
        )*
    };
}

pub async fn register_rpc_methods<C: Context>(ctx: &C) -> RpcServer<C> {
    let server = RpcServer::new(ctx.clone());
    register_rpc_method!(server, [
        eth::EthBlockNumber,
        eth::EthCall,
        eth::EthChainId,
        eth::EthEstimateGas,
        eth::EthFeeHistory,
        eth::EthGasPrice,
        eth::EthGetBalance,
        eth::EthGetBlockByHash,
        eth::EthGetBlockByNumber,
        eth::EthGetCode,
        eth::EthGetTransactionByHash,
        eth::EthGetTransactionCount,
        eth::EthGetTransactionReceipt,
        eth::EthSendRawTransaction,
        eth::EthGetLogs,
        eth::EthGetStorageAt,
        eth::NetVersion,
        SendEncryptedTx,
        SendRawTransaction
    ])
}
