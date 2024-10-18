use std::{fs, path::PathBuf, sync::Arc};

use json_rpc::RpcServer;
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
use secure_rpc::{
    cli::{Cli, Commands, Config, ConfigPath},
    client::KeyManagementSystemClient,
    error::Error,
    rpc::*,
    state::{AppState, PvdeParams},
};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut cli = Cli::init();

    match cli.command {
        Commands::Init { ref config_path } => ConfigPath::init(config_path)?,
        Commands::Start {
            ref mut config_option,
        } => {
            tracing_subscriber::fmt().init();
            std::panic::set_hook(Box::new(|panic_info| tracing::error!("{}", panic_info)));

            let config = Config::load(config_option)?;
            let config_path = config_option.path.clone();

            tracing::info!("Successfully loaded the configuration file.",);

            let is_using_zkp = config.is_using_zkp();

            let key_management_system_rpc_url = config.key_management_system_rpc_url();
            let key_management_system_client =
                KeyManagementSystemClient::new(key_management_system_rpc_url)?;

            let skde_params = key_management_system_client
                .get_skde_params()
                .await?
                .skde_params;

            let app_state = Arc::new(AppState::new(
                config,
                skde_params,
                Some(key_management_system_client),
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
    let secure_rpc_url = context.config().secure_rpc_url().to_string();

    // Initialize the external RPC server.
    let external_rpc_server = RpcServer::new(context.clone())
        // eth
        .register_rpc_method(
            eth::EthBlockNumber::METHOD_NAME,
            eth::EthBlockNumber::handler,
        )?
        .register_rpc_method(eth::EthCall::METHOD_NAME, eth::EthCall::handler)?
        .register_rpc_method(eth::EthChainId::METHOD_NAME, eth::EthChainId::handler)?
        .register_rpc_method(
            eth::EthEstimateGas::METHOD_NAME,
            eth::EthEstimateGas::handler,
        )?
        .register_rpc_method(eth::EthGasPrice::METHOD_NAME, eth::EthGasPrice::handler)?
        .register_rpc_method(eth::EthGetBalance::METHOD_NAME, eth::EthGetBalance::handler)?
        .register_rpc_method(
            eth::EthGetBlockByNumber::METHOD_NAME,
            eth::EthGetBlockByNumber::handler,
        )?
        .register_rpc_method(eth::EthGetCode::METHOD_NAME, eth::EthGetCode::handler)?
        .register_rpc_method(
            eth::EthGetTransactionCount::METHOD_NAME,
            eth::EthGetTransactionCount::handler,
        )?
        .register_rpc_method(
            eth::EthGetTransactionReceipt::METHOD_NAME,
            eth::EthGetTransactionReceipt::handler,
        )?
        .register_rpc_method(eth::EthNetVersion::METHOD_NAME, eth::EthNetVersion::handler)?
        .register_rpc_method(
            eth::EthSendRawTransaction::METHOD_NAME,
            eth::EthSendRawTransaction::handler,
        )?
        .register_rpc_method(
            eth::EthGetBlockByHash::METHOD_NAME,
            eth::EthGetBlockByHash::handler,
        )?
        // cryptography
        .register_rpc_method(EncryptTransaction::METHOD_NAME, EncryptTransaction::handler)?
        .register_rpc_method(DecryptTransaction::METHOD_NAME, DecryptTransaction::handler)?
        // sequencer
        .register_rpc_method(
            RequestToSendEncryptedTransaction::METHOD_NAME,
            RequestToSendEncryptedTransaction::handler,
        )?
        .register_rpc_method(
            RequestToSendRawTransaction::METHOD_NAME,
            RequestToSendRawTransaction::handler,
        )?
        .init(secure_rpc_url.clone())
        .await?;

    tracing::info!(
        "Successfully started the Secure RPC server: {}",
        secure_rpc_url
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
