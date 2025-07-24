use clap::{Parser, Subcommand};
use radius_sdk::{
    json_rpc::server::RpcServer,
    util::{get_resource_limit, set_resource_limit, ResourceType},
};
use secure_rpc::{
    client::distributed_key_generation::DistributedKeyGenerationClient,
    error::Error,
    rpc::{eth, *},
    state::AppState,
    types::config::{Config, ConfigOption, ConfigPath},
    websocket::run_websocket_server,
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Deserialize, Parser, Serialize)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}

#[derive(Subcommand, Debug, Deserialize, Serialize)]
pub enum Commands {
    /// Initializes a node
    Init {
        #[clap(flatten)]
        config_path: Box<ConfigPath>,
    },

    /// Starts the node
    Start {
        #[clap(flatten)]
        config_option: Box<ConfigOption>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();
    std::panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let location = panic_info.location();

        if let Some(panic_log) = payload.downcast_ref::<&'static str>() {
            tracing::error!("{:?} at {:?}", panic_log, location);
        } else if let Some(panic_log) = payload.downcast_ref::<String>() {
            tracing::error!("{:?} at {:?}", panic_log, location);
        } else {
            tracing::error!("Panic at {:?}", location);
        }
    }));

    let mut cli = Cli::init();

    match cli.command {
        Commands::Init { ref config_path } => ConfigPath::init(config_path)?,
        Commands::Start {
            ref mut config_option,
        } => {
            let rlimit = get_resource_limit(ResourceType::RLIMIT_NOFILE)?;
            set_resource_limit(ResourceType::RLIMIT_NOFILE, rlimit.hard_limit)?;

            let config = Config::load(config_option)?;

            tracing::info!(
                "Successfully loaded the configuration file. config: {:?}",
                config
            );

            let distributed_key_generation_rpc_url = config.distributed_key_generation_rpc_url();
            let distributed_key_generation_client =
                DistributedKeyGenerationClient::new(distributed_key_generation_rpc_url)
                    .map_err(Error::DistributedKeyGenerationClient)?;

            tracing::info!("Successfully initialize distributed key generation client.");

            let skde_params = distributed_key_generation_client
                .get_skde_params()
                .await
                .map_err(Error::DistributedKeyGenerationClient)?
                .skde_params;

            tracing::info!("Complete to skde params: {:?}", skde_params);

            let app_state =
                AppState::new(config, skde_params, Some(distributed_key_generation_client))?;

            run_websocket_server(app_state.clone()).await;
            // Initialize the secure RPC server.
            let server_handle = initialize_external_rpc_server(app_state).await?;

            server_handle.await.unwrap();
        }
    }

    Ok(())
}

async fn initialize_external_rpc_server(
    context: AppState, // rpc_client: &RpcClient,
) -> Result<JoinHandle<()>, Error> {
    let external_rpc_url = anywhere(&context.config().external_rpc_port()?);

    // Initialize the external RPC server.
    let external_rpc_server = RpcServer::new(context.clone())
        // eth
        .register_rpc_method::<eth::EthBlockNumber>()?
        .register_rpc_method::<eth::EthChainId>()?
        .register_rpc_method::<eth::EthProtocolVersion>()?
        .register_rpc_method::<eth::EthSyncing>()?
        .register_rpc_method::<eth::EthGasPrice>()?
        .register_rpc_method::<eth::EthMaxPriorityFeePerGas>()?
        .register_rpc_method::<eth::EthFeeHistory>()?
        .register_rpc_method::<eth::EthGetBlockByHash>()?
        .register_rpc_method::<eth::EthGetBlockByNumber>()?
        .register_rpc_method::<eth::EthGetBlockTransactionCountByHash>()?
        .register_rpc_method::<eth::EthGetBlockTransactionCountByNumber>()?
        .register_rpc_method::<eth::EthGetUncleByBlockHashAndIndex>()?
        .register_rpc_method::<eth::EthGetUncleByBlockNumberAndIndex>()?
        .register_rpc_method::<eth::EthGetUncleCountByBlockHash>()?
        .register_rpc_method::<eth::EthGetUncleCountByBlockNumber>()?
        .register_rpc_method::<eth::EthGetTransactionByHash>()?
        .register_rpc_method::<eth::EthGetRawTransactionByHash>()?
        .register_rpc_method::<eth::EthGetTransactionByBlockHashAndIndex>()?
        .register_rpc_method::<eth::EthRetRawTransactionByBlockHashAndIndex>()?
        .register_rpc_method::<eth::EthGetTransactionByBlockNumberAndIndex>()?
        .register_rpc_method::<eth::EthRetRawTransactionByBlockNumberAndIndex>()?
        .register_rpc_method::<eth::EthGetTransactionReceipt>()?
        .register_rpc_method::<eth::EthGetBlockReceipts>()?
        .register_rpc_method::<eth::EthEstimateGas>()?
        .register_rpc_method::<eth::EthGetBalance>()?
        .register_rpc_method::<eth::EthGetCode>()?
        .register_rpc_method::<eth::EthGetTransactionCount>()?
        .register_rpc_method::<eth::EthGetStorageAt>()?
        .register_rpc_method::<eth::EthCall>()?
        .register_rpc_method::<eth::EthCallMany>()?
        .register_rpc_method::<eth::EthCallBundle>()?
        .register_rpc_method::<eth::EthCreateAccessList>()?
        .register_rpc_method::<eth::EthNewFilter>()?
        .register_rpc_method::<eth::EthNewBlockFilter>()?
        .register_rpc_method::<eth::EthNewPendingTransactionFilter>()?
        .register_rpc_method::<eth::EthGetFilterLogs>()?
        .register_rpc_method::<eth::EthGetFilterChanges>()?
        .register_rpc_method::<eth::EthUninstallFilter>()?
        .register_rpc_method::<eth::EthGetLogs>()?
        .register_rpc_method::<eth::EthSignTypedData>()?
        .register_rpc_method::<eth::EthGetProof>()?
        .register_rpc_method::<eth::EthMining>()?
        .register_rpc_method::<eth::EthCoinbase>()?
        .register_rpc_method::<eth::EthHashrate>()?
        .register_rpc_method::<eth::EthSubmitHashrate>()?
        .register_rpc_method::<eth::EthGetWork>()?
        .register_rpc_method::<eth::EthSubmitWork>()?
        .register_rpc_method::<eth::EthSubscribe>()?
        .register_rpc_method::<eth::EthUnsubscribe>()?
        .register_rpc_method::<eth::EthSendRawTransaction>()?
        // zkevm
        .register_rpc_method::<eth::ZkevmBatchNumber>()?
        .register_rpc_method::<eth::ZkevmBatchNumberByBlockNumber>()?
        .register_rpc_method::<eth::ZkevmConsolidatedBlockNumber>()?
        .register_rpc_method::<eth::ZkevmEstimateCounters>()?
        .register_rpc_method::<eth::ZkevmGetBatchByNumber>()?
        .register_rpc_method::<eth::ZkevmGetBatchCountersByNumber>()?
        .register_rpc_method::<eth::ZkevmGetBatchWitness>()?
        .register_rpc_method::<eth::ZkevmGetBlockRangeWitness>()?
        .register_rpc_method::<eth::ZkevmGetExitRootTable>()?
        .register_rpc_method::<eth::ZkevmGetExitRootsByGER>()?
        .register_rpc_method::<eth::ZkevmGetForkById>()?
        .register_rpc_method::<eth::ZkevmGetForkId>()?
        .register_rpc_method::<eth::ZkevmGetForkIdByBatchNumber>()?
        .register_rpc_method::<eth::ZkevmGetForks>()?
        .register_rpc_method::<eth::ZkevmGetFullBlockByHash>()?
        .register_rpc_method::<eth::ZkevmGetFullBlockByNumber>()?
        .register_rpc_method::<eth::ZkevmGetL2BlockInfoTree>()?
        .register_rpc_method::<eth::ZkevmGetLatestDataStreamBlock>()?
        .register_rpc_method::<eth::ZkevmGetLatestGlobalExitRoot>()?
        .register_rpc_method::<eth::ZkevmGetProverInput>()?
        .register_rpc_method::<eth::ZkevmGetRollupAddress>()?
        .register_rpc_method::<eth::ZkevmGetRollupManagerAddress>()?
        .register_rpc_method::<eth::ZkevmGetVersionHistory>()?
        .register_rpc_method::<eth::ZkevmGetWitness>()?
        .register_rpc_method::<eth::ZkevmIsBlockConsolidated>()?
        .register_rpc_method::<eth::ZkevmIsBlockVirtualized>()?
        .register_rpc_method::<eth::ZkevmVerifiedBatchNumber>()?
        .register_rpc_method::<eth::ZkevmVirtualBatchNumber>()?
        // net
        .register_rpc_method::<eth::NetVersion>()?
        // cryptography
        .register_rpc_method::<DecryptTransaction>()?
        .register_rpc_method::<EncryptTransaction>()?
        // tx_orderer
        .register_rpc_method::<SendEncryptedTransaction>()?
        .register_rpc_method::<SendRawTransaction>()?
        .init(external_rpc_url.clone())
        .await?;

    tracing::info!(
        "Successfully started the Secure RPC server: {}",
        external_rpc_url
    );

    let server_handle = tokio::spawn(async move {
        external_rpc_server.stopped().await;
    });

    Ok(server_handle)
}

pub fn anywhere(port: &str) -> String {
    format!("0.0.0.0:{}", port)
}
