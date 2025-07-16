use secure_rpc_primitives::Context;
use radius_sdk::json_rpc::server::RpcServer;
use secure_rpc_service::*;

pub async fn default_external_rpc_server<C: Context>(ctx: &C) -> RpcServer<C> {
    RpcServer::new(ctx.clone())
        .register_rpc_method::<eth::EthBlockNumber>()?
        .register_rpc_method::<eth::EthCall>()?
        .register_rpc_method::<eth::EthChainId>()?
        .register_rpc_method::<eth::EthEstimateGas>()?
        .register_rpc_method::<eth::EthFeeHistory>()?
        .register_rpc_method::<eth::EthGasPrice>()?
        .register_rpc_method::<eth::EthGetBalance>()?
        .register_rpc_method::<eth::EthGetBlockByHash>()?
        .register_rpc_method::<eth::EthGetBlockByNumber>()?
        .register_rpc_method::<eth::EthGetCode>()?
        .register_rpc_method::<eth::EthGetTransactionByHash>()?
        .register_rpc_method::<eth::EthGetTransactionCount>()?
        .register_rpc_method::<eth::EthGetTransactionReceipt>()?
        .register_rpc_method::<eth::EthSendRawTransaction>()?
        .register_rpc_method::<eth::EthGetLogs>()?
        .register_rpc_method::<eth::EthGetStorageAt>()?
        .register_rpc_method::<eth::NetVersion>()?
        // cryptography
        .register_rpc_method::<DecryptTransaction>()?
        .register_rpc_method::<EncryptTransaction>()?
        // tx_orderer
        .register_rpc_method::<SendEncryptedTransaction>()?
        .register_rpc_method::<SendRawTransaction>()
        .unwrap()
}
