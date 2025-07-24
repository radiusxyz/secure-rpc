mod rpc;
mod worker;
use tokio::task::JoinHandle;
use worker::{external_rpc::start_external_rpc_worker, operator::start_operator_worker, secure_rpc::start_secure_rpc_worker};
use secure_rpc_node_primitive::{NodeConfig, SecureRpcNode, SkdeSecureRpcService, BlockchainService, ExternalRpcService};

pub async fn run_node(config: NodeConfig) -> anyhow::Result<()> {
    config.log_config();
    let NodeConfig {
        rpc_url,
        tx_orderer_rpc_url,
        rollup_rpc_url,
        rollup_id,
        blockchain_url,
        contract_address,
        encrypt_mode,
        ..
    } = config;

    // Create the secure RPC node for the context provider
    let secure_rpc_node = SecureRpcNode::<SkdeSecureRpcService, ExternalRpcService>::new(
        rollup_id.clone(), 
        encrypt_mode
    );

    let handles = start_workers(secure_rpc_node, &blockchain_url, &contract_address, &rpc_url, &rollup_rpc_url, &tx_orderer_rpc_url, &rollup_id).await?;

    futures::future::join_all(handles).await;

    Ok(())
}

pub async fn start_workers(secure_rpc_node: SecureRpcNode<SkdeSecureRpcService, ExternalRpcService>, blockchain_url: &str, contract_address: &str, rpc_url: &str, rollup_rpc_url: &str, tx_orderer_rpc_url: &str, rollup_id: &str) -> anyhow::Result<Vec<JoinHandle<()>>> {
    let mut handles = vec![];
    let (blockchain_handle, blockchain_state_event_rx, trusted_setup, dkg_rpc_urls) = start_operator_worker::<_, _>(&secure_rpc_node, BlockchainService::new(&blockchain_url, &contract_address)).await?;
    let (external_rpc_service, rpc_handle) = start_external_rpc_worker(dkg_rpc_urls, rollup_rpc_url, tx_orderer_rpc_url).await;
    let (secure_rpc_handle, rpc_event_tx) = start_secure_rpc_worker(&secure_rpc_node, blockchain_state_event_rx).await;
    // Register RPC methods and start the server
    let rpc_server = rpc::register_rpc_methods(&secure_rpc_node).await.init(rpc_url.clone()).await?; 
    handles.push(tokio::spawn(async move { rpc_server.stopped().await;}));
    tracing::info!("🚀 RPC server started successfully on {}", rpc_url);
    handles.push(blockchain_handle);
    handles.push(rpc_handle);
    handles.push(secure_rpc_handle);
    Ok(handles)
}

