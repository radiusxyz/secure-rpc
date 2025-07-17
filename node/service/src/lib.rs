mod rpc;
mod worker;
use tokio::task::JoinHandle;
use worker::rpc::start_external_rpc_worker;
use secure_rpc_node_primitive::{NodeConfig, SecureRPCNode, SkdeSecureRPCService, BlockchainService, service};
use skde::delay_encryption::SkdeParams;

pub async fn get_skde_params(config: NodeConfig) -> anyhow::Result<SkdeParams> {
    let blockchain_service = BlockchainService::new(&config.blockchain_url, &config.contract_address);
    let skde_params = blockchain_service.get_trusted_setup().await?;
    Ok(skde_params)
}

pub async fn run_node(config: NodeConfig) -> anyhow::Result<()> {
    let mut handle: Vec<JoinHandle<_>> = vec![];
    let (external_rpc_service, rpc_handle) = start_external_rpc_worker(&config.dkg_rpc_url, &config.rollup_rpc_url, vec![config.tx_orderer_rpc_url.clone()]).await;
    handle.push(rpc_handle);
    let skde_params = get_skde_params(config.clone()).await?;
    let secure_rpc_node = SecureRPCNode::new(config.rollup_id.clone(), SkdeSecureRPCService::new(skde_params), external_rpc_service, config.is_encrypt_enabled());
    let rpc_server = rpc::register_rpc_methods(&secure_rpc_node).await.init(config.external_rpc_url).await?; 
    handle.push(tokio::spawn(async move { rpc_server.stopped().await;}));

    futures::future::join_all(handle).await;

    Ok(())
}

