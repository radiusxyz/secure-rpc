use std::{fs, path::PathBuf, sync::Arc};

use clap::{Parser, Subcommand};
use pvde::{
    encryption::poseidon_encryption_zkp::{
        export_proving_key as export_poseidon_encryption_proving_key,
        export_verifying_key as export_poseidon_encryption_verifying_key,
        export_zkp_param as export_poseidon_encryption_zkp_param,
        import_proving_key as import_poseidon_encryption_proving_key,
        import_verifying_key as import_poseidon_encryption_verifying_key,
        import_zkp_param as import_poseidon_encryption_zkp_param,
        setup as setup_poseidon_encryption,
    },
    time_lock_puzzle::{
        export_time_lock_puzzle_param, import_time_lock_puzzle_param,
        key_validation_zkp::{
            export_proving_key as export_key_validation_proving_key,
            export_verifying_key as export_key_validation_verifying_key,
            export_zkp_param as export_key_validation_zkp_param,
            import_proving_key as import_key_validation_proving_key,
            import_verifying_key as import_key_validation_verifying_key,
            import_zkp_param as import_key_validation_zkp_param, setup as setup_key_validation,
        },
        setup as setup_time_lock_puzzle_param,
    },
};
use radius_sdk::{
    json_rpc::server::RpcServer,
    util::{get_resource_limit, set_resource_limit, ResourceType},
};
use secure_rpc::{
    client::distributed_key_generation::DistributedKeyGenerationClient,
    error::Error,
    rpc::{eth, *},
    state::{AppState, PvdeParams},
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
            let config_path = config_option.path.clone();

            tracing::info!("Successfully loaded the configuration file.",);

            let is_using_zkp = config.is_using_zkp();

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

            let app_state = Arc::new(AppState::new(
                config,
                skde_params,
                Some(distributed_key_generation_client),
            ));

            // Initialize the secure RPC server.
            let server_handle = initialize_external_rpc_server(&app_state).await?;

            if let Some(path) = config_path {
                // Initialize the time lock puzzle parameters.
                store_time_lock_puzzle_param(app_state, path, is_using_zkp).await?;
            }

            server_handle.await.unwrap();
        }
    }

    Ok(())
}

async fn initialize_external_rpc_server(
    context: &AppState, // rpc_client: &RpcClient,
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

pub async fn store_time_lock_puzzle_param(
    app_state: Arc<AppState>,
    config_path: PathBuf,
    is_using_zkp: bool,
) -> Result<(), Error> {
    let time_lock_puzzle_param_path = config_path
        .join("time_lock_puzzle_param.json")
        .to_str()
        .unwrap()
        .to_string();

    let time_lock_puzzle_param = if fs::metadata(&time_lock_puzzle_param_path).is_ok() {
        import_time_lock_puzzle_param(&time_lock_puzzle_param_path)
    } else {
        let time_lock_puzzle_param = setup_time_lock_puzzle_param(2048);
        export_time_lock_puzzle_param(&time_lock_puzzle_param_path, time_lock_puzzle_param.clone());
        time_lock_puzzle_param
    };

    let mut pvde_params = PvdeParams::default();
    pvde_params.update_time_lock_puzzle_param(time_lock_puzzle_param);

    if is_using_zkp {
        let key_validation_param_file_path = config_path
            .join("key_validation_zkp_param.data")
            .to_str()
            .unwrap()
            .to_string();
        let key_validation_proving_key_file_path = config_path
            .join("key_validation_proving_key.data")
            .to_str()
            .unwrap()
            .to_string();
        let key_validation_verifying_key_file_path = config_path
            .join("key_validation_verifying_key.data")
            .to_str()
            .unwrap()
            .to_string();

        let (key_validation_zkp_param, key_validation_verifying_key, key_validation_proving_key) =
            if fs::metadata(&key_validation_param_file_path).is_ok() {
                (
                    import_key_validation_zkp_param(&key_validation_param_file_path),
                    import_key_validation_verifying_key(&key_validation_verifying_key_file_path),
                    import_key_validation_proving_key(&key_validation_proving_key_file_path),
                )
            } else {
                let setup_results = setup_key_validation(13);
                export_key_validation_zkp_param(
                    &key_validation_param_file_path,
                    setup_results.0.clone(),
                );
                export_key_validation_verifying_key(
                    &key_validation_verifying_key_file_path,
                    setup_results.1.clone(),
                );
                export_key_validation_proving_key(
                    &key_validation_proving_key_file_path,
                    setup_results.2.clone(),
                );
                setup_results
            };

        pvde_params.update_key_validation_zkp_param(key_validation_zkp_param);
        pvde_params.update_key_validation_proving_key(key_validation_proving_key);
        pvde_params.update_key_validation_verifying_key(key_validation_verifying_key);

        let poseidon_encryption_param_file_path = config_path
            .join("poseidon_encryption_param.json")
            .to_str()
            .unwrap()
            .to_string();
        let poseidon_encryption_proving_key_file_path = config_path
            .join("poseidon_encryption_proving_key.data")
            .to_str()
            .unwrap()
            .to_string();
        let poseidon_encryption_verifying_key_file_path = config_path
            .join("poseidon_encryption_verifying_key.data")
            .to_str()
            .unwrap()
            .to_string();

        let (
            poseidon_encryption_zkp_param,
            poseidon_encryption_verifying_key,
            poseidon_encryption_proving_key,
        ) = if fs::metadata(&poseidon_encryption_param_file_path).is_ok() {
            (
                import_poseidon_encryption_zkp_param(&poseidon_encryption_param_file_path),
                import_poseidon_encryption_verifying_key(
                    &poseidon_encryption_verifying_key_file_path,
                ),
                import_poseidon_encryption_proving_key(&poseidon_encryption_proving_key_file_path),
            )
        } else {
            let setup_results = setup_poseidon_encryption(13);
            export_poseidon_encryption_zkp_param(
                &poseidon_encryption_param_file_path,
                setup_results.0.clone(),
            );
            export_poseidon_encryption_verifying_key(
                &poseidon_encryption_verifying_key_file_path,
                setup_results.1.clone(),
            );
            export_poseidon_encryption_proving_key(
                &poseidon_encryption_proving_key_file_path,
                setup_results.2.clone(),
            );
            setup_results
        };

        pvde_params.update_poseidon_encryption_zkp_param(poseidon_encryption_zkp_param);
        pvde_params.update_poseidon_encryption_proving_key(poseidon_encryption_proving_key);
        pvde_params.update_poseidon_encryption_verifying_key(poseidon_encryption_verifying_key);

        app_state
            .pvde_params()
            .update(Some(pvde_params))
            .map_err(|error| {
                tracing::error!("Failed to update the PVDE parameters: {:?}", error);
                Error::ContextUpdateFail
            })?;
    }

    Ok(())
}

pub fn anywhere(port: &str) -> String {
    format!("0.0.0.0:{}", port)
}
