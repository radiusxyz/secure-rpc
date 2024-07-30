use std::{env, fs, path::PathBuf, sync::Arc};

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
    cli::{Cli, Commands, Config, ConfigOption, ConfigPath},
    error::Error,
    rpc::external::{self, RollupRpcParameter},
    state::AppState,
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

            // let arguments: Vec<String> = env::args().skip(1).collect();
            // let config_path = arguments
            //     .get(0)
            //     .expect("Provide the config file path.")
            //     .to_owned();

            tracing::info!("Successfully loaded the configuration file.",);

            let is_using_zkp = config.is_using_zkp();

            let app_state = Arc::new(AppState::new(config));

            // Initialize the secure RPC server.
            let server_handle = initialize_external_rpc_server(&app_state).await?;

            if is_using_zkp {
                // Initialize the time lock puzzle parameters.
                store_time_lock_puzzle_param().await?;
            }

            server_handle.await.unwrap();
        }
    }

    Ok(())
}

async fn initialize_external_rpc_server(
    app_state: &AppState, // rpc_client: &RpcClient,
) -> Result<JoinHandle<()>, Error> {
    // Initialize the external RPC server.
    let secure_rpc_server = RpcServer::new(app_state.clone())
        .register_rpc_method(
            external::eth::EthBlockNumber::METHOD_NAME,
            external::eth::EthBlockNumber::handler,
        )?
        .register_rpc_method(
            external::eth::EthCall::METHOD_NAME,
            external::eth::EthCall::handler,
        )?
        .register_rpc_method(
            external::eth::EthChainId::METHOD_NAME,
            external::eth::EthChainId::handler,
        )?
        .register_rpc_method(
            external::eth::EthEstimateGas::METHOD_NAME,
            external::eth::EthEstimateGas::handler,
        )?
        .register_rpc_method(
            external::eth::EthGasPrice::METHOD_NAME,
            external::eth::EthGasPrice::handler,
        )?
        .register_rpc_method(
            external::eth::EthGetBalance::METHOD_NAME,
            external::eth::EthGetBalance::handler,
        )?
        .register_rpc_method(
            external::eth::EthGetBlockByNumber::METHOD_NAME,
            external::eth::EthGetBlockByNumber::handler,
        )?
        .register_rpc_method(
            external::eth::EthGetCode::METHOD_NAME,
            external::eth::EthGetCode::handler,
        )?
        .register_rpc_method(
            external::eth::EthGetTransactionCount::METHOD_NAME,
            external::eth::EthGetTransactionCount::handler,
        )?
        .register_rpc_method(
            external::eth::EthGetTransactionReceipt::METHOD_NAME,
            external::eth::EthGetTransactionReceipt::handler,
        )?
        .register_rpc_method(
            external::eth::EthNetVersion::METHOD_NAME,
            external::eth::EthNetVersion::handler,
        )?
        .register_rpc_method(
            external::eth::EthSendRawTransaction::METHOD_NAME,
            external::eth::EthSendRawTransaction::handler,
        )?
        // .register_rpc_method(
        //     external::SendTransaction::METHOD_NAME,
        //     external::SendTransaction::handler,
        // )?
        // .register_rpc_method(
        //     external::SendRawTransaction::METHOD_NAME,
        //     external::SendRawTransaction::handler,
        // )?
        // .register_rpc_method(
        //     external::SendEncryptedTransaction::METHOD_NAME,
        //     external::SendEncryptedTransaction::handler,
        // )?
        .init(app_state.config().secure_rpc_url())
        .await?;

    tracing::info!(
        "Successfully started the Secure RPC server: {}",
        app_state.config().secure_rpc_url()
    );

    Ok(tokio::spawn(async move {
        secure_rpc_server.stopped().await;
    }))

    // server_handle.await.unwrap();

    // Ok(())
}

pub async fn store_time_lock_puzzle_param() -> Result<(), Error> {
    // let config_path = PathBuf::from(config_path);
    // let time_lock_puzzle_param_path = config_path
    //     .join("time_lock_puzzle_param.json")
    //     .to_str()
    //     .unwrap()
    //     .to_string();

    // let time_lock_puzzle_param = if fs::metadata(&time_lock_puzzle_param_path).is_ok() {
    //     import_time_lock_puzzle_param(&time_lock_puzzle_param_path)
    // } else {
    //     let time_lock_puzzle_param = setup_time_lock_puzzle_param(2048);
    //     export_time_lock_puzzle_param(&time_lock_puzzle_param_path, time_lock_puzzle_param.clone());
    //     time_lock_puzzle_param
    // };

    // if do_verify_tx_with_zkp {
    //     let key_validation_param_file_path = config_path
    //         .join("key_validation_zkp_param.data")
    //         .to_str()
    //         .unwrap()
    //         .to_string();
    //     let key_validation_proving_key_file_path = config_path
    //         .join("key_validation_proving_key.data")
    //         .to_str()
    //         .unwrap()
    //         .to_string();
    //     let key_validation_verifying_key_file_path = config_path
    //         .join("key_validation_verifying_key.data")
    //         .to_str()
    //         .unwrap()
    //         .to_string();

    //     let (key_validation_zkp_param, key_validation_verifying_key, key_validation_proving_key) =
    //         if fs::metadata(&key_validation_param_file_path).is_ok() {
    //             (
    //                 import_key_validation_zkp_param(&key_validation_param_file_path),
    //                 import_key_validation_verifying_key(&key_validation_verifying_key_file_path),
    //                 import_key_validation_proving_key(&key_validation_proving_key_file_path),
    //             )
    //         } else {
    //             let setup_results = setup_key_validation(13);
    //             export_key_validation_zkp_param(
    //                 &key_validation_param_file_path,
    //                 setup_results.0.clone(),
    //             );
    //             export_key_validation_verifying_key(
    //                 &key_validation_verifying_key_file_path,
    //                 setup_results.1.clone(),
    //             );
    //             export_key_validation_proving_key(
    //                 &key_validation_proving_key_file_path,
    //                 setup_results.2.clone(),
    //             );
    //             setup_results
    //         };

    //     let poseidon_encryption_param_file_path = config_path
    //         .join("poseidon_encryption_param.json")
    //         .to_str()
    //         .unwrap()
    //         .to_string();
    //     let poseidon_encryption_proving_key_file_path = config_path
    //         .join("poseidon_encryption_proving_key.data")
    //         .to_str()
    //         .unwrap()
    //         .to_string();
    //     let poseidon_encryption_verifying_key_file_path = config_path
    //         .join("poseidon_encryption_verifying_key.data")
    //         .to_str()
    //         .unwrap()
    //         .to_string();

    //     let (
    //         poseidon_encryption_zkp_param,
    //         poseidon_encryption_verifying_key,
    //         poseidon_encryption_proving_key,
    //     ) = if fs::metadata(&poseidon_encryption_param_file_path).is_ok() {
    //         (
    //             import_poseidon_encryption_zkp_param(&poseidon_encryption_param_file_path),
    //             import_poseidon_encryption_verifying_key(
    //                 &poseidon_encryption_verifying_key_file_path,
    //             ),
    //             import_poseidon_encryption_proving_key(&poseidon_encryption_proving_key_file_path),
    //         )
    //     } else {
    //         let setup_results = setup_poseidon_encryption(13);
    //         export_poseidon_encryption_zkp_param(
    //             &poseidon_encryption_param_file_path,
    //             setup_results.0.clone(),
    //         );
    //         export_poseidon_encryption_verifying_key(
    //             &poseidon_encryption_verifying_key_file_path,
    //             setup_results.1.clone(),
    //         );
    //         export_poseidon_encryption_proving_key(
    //             &poseidon_encryption_proving_key_file_path,
    //             setup_results.2.clone(),
    //         );
    //         setup_results
    //     };
    // }

    Ok(())
}
