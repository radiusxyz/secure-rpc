mod rpc;
mod worker;
use tokio::task::JoinHandle;
use worker::rpc::start_external_rpc_worker;
use secure_rpc_node_primitive::{NodeConfig, SecureRpcNode, SkdeSecureRpcService};

pub async fn run_node(config: NodeConfig) -> anyhow::Result<()> {
    config.log_config();
    let mut handle: Vec<JoinHandle<_>> = vec![];
    let (external_rpc_service, rpc_handle) = start_external_rpc_worker(&config.dkg_rpc_url, &config.rollup_rpc_url, vec![config.tx_orderer_rpc_url.clone()]).await;
    handle.push(rpc_handle);
    let secure_rpc_node = SecureRpcNode::new(
        config.rollup_id.clone(), 
        SkdeSecureRpcService::new(&config.blockchain_url, &config.contract_address), 
        external_rpc_service, 
        config.is_encrypt_enabled()
    );
    let rpc_server = rpc::register_rpc_methods(&secure_rpc_node).await.init(config.rpc_url).await?; 
    handle.push(tokio::spawn(async move { rpc_server.stopped().await;}));

    futures::future::join_all(handle).await;

    Ok(())
}

