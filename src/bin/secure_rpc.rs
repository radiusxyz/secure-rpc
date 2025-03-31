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

            tracing::info!("Successfully loaded the configuration file.",);

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
    let external_rpc_url = anywhere(&context.config().external_port()?);

    // Initialize the external RPC server.
    let external_rpc_server = RpcServer::new(context.clone())
        // eth
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
        .register_rpc_method::<eth::EthNetVersion>()?
        .register_rpc_method::<eth::EthSendRawTransaction>()?
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
