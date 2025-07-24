mod rpc;
mod worker;
use alloy::{primitives::Address as EthAddress, providers::{ProviderBuilder, WsConnect}, transports::http::reqwest::Url};
use tokio::task::JoinHandle;
use worker::{external_rpc::start_external_rpc_worker, operator::start_operator_worker, secure_rpc::start_secure_rpc_worker};
use secure_rpc_node_primitive::{
    NodeConfig, SecureRpcNode, SkdeSecureRpcService, 
    basic::BasicOperatorService, 
    blockchain::{BlockchainService, OperatorContractInstance}, TaskExecutor
};
use secure_rpc_primitives::{AsyncTask, ExternalRpcInterface, SecureRpcService};


fn create_secure_rpc_node<S, E, AT>(config: &NodeConfig) -> SecureRpcNode<S, E, AT> 
where
    S: SecureRpcService,
    E: ExternalRpcInterface,
    AT: AsyncTask<S::EncryptedTx>,
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
    AT: AsyncTask<S::EncryptedTx>,
{
    let rpc_server = rpc::register_rpc_methods(secure_rpc_node).await.init(rpc_url).await?; 
    let handle = tokio::spawn(async move { rpc_server.stopped().await;});
    tracing::info!("🚀 RPC server started successfully on {}", rpc_url);
    Ok(handle)
}

pub async fn run_blockchain_operator_secure_rpc_node(config: NodeConfig, blockchain_http_rpc_url: String, blockchain_ws_rpc_url: String, contract_address: String) -> anyhow::Result<()> {
    let mut handles = vec![];
    let mut secure_rpc_node = create_secure_rpc_node::<_, _, _>(&config);

    // Use blockchain configuration from args
    let http_provider = ProviderBuilder::new().on_http(Url::parse(&blockchain_http_rpc_url).unwrap());
    let ws_provider = ProviderBuilder::new().on_ws(WsConnect::new(&blockchain_ws_rpc_url)).await.unwrap();
    let contract_instance = OperatorContractInstance::new(contract_address.parse::<EthAddress>().unwrap(), http_provider);
    let blockchain_service = BlockchainService::new(ws_provider, contract_instance);

    // Start the operator worker
    let (operator_handle, operator_event_rx, trusted_setup, dkg_rpc_urls) = start_operator_worker::<_, _>(&secure_rpc_node, blockchain_service).await?;

    // Start the external RPC worker
    let (external_rpc_service, rpc_handle) = start_external_rpc_worker(dkg_rpc_urls, &config.rollup_rpc_url, &config.tx_orderer_rpc_url).await;

    secure_rpc_node.with_secure_rpc_service(SkdeSecureRpcService::new(trusted_setup));
    secure_rpc_node.with_external_rpc_service(external_rpc_service);

    // Start the secure RPC worker
    let (secure_rpc_handle, rpc_event_tx) = start_secure_rpc_worker(&secure_rpc_node, operator_event_rx).await;
    secure_rpc_node.with_async_task(TaskExecutor::new(rpc_event_tx));

    let rpc_server_handle = run_rpc_server(&secure_rpc_node, &config.rpc_url).await?;

    handles.push(operator_handle);
    handles.push(rpc_handle);
    handles.push(secure_rpc_handle);
    handles.push(rpc_server_handle);

    futures::future::join_all(handles).await;

    Ok(())
}

pub async fn run_basic_operator_secure_rpc_node(config: NodeConfig, dkg_rpc_urls: Vec<String>) -> anyhow::Result<()> {
    let mut handles = vec![];
    let mut secure_rpc_node = create_secure_rpc_node::<_, _, _>(&config);
    tracing::info!("🔧 Starting in basic operator mode");
    tracing::info!("🔗 DKG RPC URLs: {:?}", dkg_rpc_urls);
    
    // Start the external RPC worker with the DKG URLs
    let (external_rpc_service, rpc_handle) = start_external_rpc_worker(dkg_rpc_urls.clone(), &config.rollup_rpc_url, &config.tx_orderer_rpc_url).await;

    // Start the operator worker
    let basic_operator_service = BasicOperatorService::new(dkg_rpc_urls.clone(), secure_rpc_node.clone());
    let (_operator_handle, operator_event_rx, trusted_setup, _dkg_rpc_urls) = start_operator_worker::<_, _>(&secure_rpc_node, basic_operator_service).await?;
    
    // Start the secure RPC worker
    let (secure_rpc_handle, rpc_event_tx) = start_secure_rpc_worker(&secure_rpc_node, operator_event_rx).await;

    let rpc_server_handle = run_rpc_server(&secure_rpc_node, &config.rpc_url).await?;

    secure_rpc_node.with_secure_rpc_service(SkdeSecureRpcService::new(trusted_setup));
    secure_rpc_node.with_external_rpc_service(external_rpc_service);
    secure_rpc_node.with_async_task(TaskExecutor::new(rpc_event_tx));
    
    handles.push(rpc_handle);
    handles.push(secure_rpc_handle);
    handles.push(rpc_server_handle);

    futures::future::join_all(handles).await;

    Ok(())
}

