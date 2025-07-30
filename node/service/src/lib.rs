mod rpc;
mod worker;
use tokio::task::JoinHandle;
use worker::{external_rpc::start_external_rpc_worker, operator::start_blockchain_operator_worker, secure_rpc::start_secure_rpc_worker};
use secure_rpc_node_primitive::{
    NodeConfig, SecureRpcNode, SkdeSecureRpcService, TaskExecutor
};
use secure_rpc_primitives::{AsyncTask, ExternalRpcInterface, SecureRpcService};


fn create_secure_rpc_node<S, E, AT>(config: &NodeConfig) -> SecureRpcNode<S, E, AT> 
where
    S: SecureRpcService,
    E: ExternalRpcInterface,
    AT: AsyncTask,
{
    let NodeConfig {
        rollup_id,
        encrypt_mode,
        ..
    } = config.clone();
    SecureRpcNode::<S, E, AT>::new(rollup_id.clone(), encrypt_mode)
}

async fn run_rpc_server<S, E, AT>(secure_rpc_node: &SecureRpcNode<S, E, AT>, rpc_url: &str) -> anyhow::Result<JoinHandle<()>> 
where
    S: SecureRpcService,
    E: ExternalRpcInterface,
    AT: AsyncTask,
{
    let rpc_server = rpc::register_rpc_methods(secure_rpc_node).await.init(rpc_url).await?; 
    let handle = tokio::spawn(async move { rpc_server.stopped().await;});
    tracing::info!("🚀 RPC server started successfully on {}", rpc_url);
    Ok(handle)
}

pub async fn run_blockchain_operator_secure_rpc_node(config: NodeConfig, blockchain_http_rpc_url: String, contract_address: String) -> anyhow::Result<()> {
    config.log_config();
    tracing::info!("Blockchain HTTP RPC URL: {}", blockchain_http_rpc_url);
    tracing::info!("Contract Address: {}", contract_address);
    let mut handles = vec![];
    let mut secure_rpc_node = create_secure_rpc_node::<_, _, _>(&config);

    // Start the operator worker
    let (operator_handle, operator_event_rx, trusted_setup, dkg_rpc_urls) = start_blockchain_operator_worker::<_>(&secure_rpc_node, blockchain_http_rpc_url, contract_address).await?;
    tracing::info!("Operator worker started successfully");
    tracing::info!("👥 Dkg operators: {:?}", dkg_rpc_urls);

    // Start the external RPC worker
    let (external_rpc_service, rpc_handle) = start_external_rpc_worker(&config.rollup_rpc_url).await;
    tracing::info!("External RPC worker started successfully");
    
    secure_rpc_node.with_secure_rpc_service(SkdeSecureRpcService::new(trusted_setup));
    secure_rpc_node.with_external_rpc_service(external_rpc_service);

    // Start the secure RPC worker
    let (secure_rpc_handle, rpc_event_tx) = start_secure_rpc_worker(&secure_rpc_node, dkg_rpc_urls, config.tx_orderer_rpc_url, operator_event_rx).await;
    secure_rpc_node.with_async_task(TaskExecutor::new(rpc_event_tx));

    let rpc_server_handle = run_rpc_server(&secure_rpc_node, &config.rpc_url).await?;

    handles.push(operator_handle);
    handles.push(rpc_handle);
    handles.push(secure_rpc_handle);
    handles.push(rpc_server_handle);

    futures::future::join_all(handles).await;

    Ok(())
}