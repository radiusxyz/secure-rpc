use std::{env, sync::Arc};

use json_rpc::RpcServer;
use secure_rpc::{
    config::Config,
    error::Error,
    rpc::external::{self, RollupRpcParameter},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();
    std::panic::set_hook(Box::new(|panic_info| tracing::error!("{}", panic_info)));

    let arguments: Vec<String> = env::args().skip(1).collect();
    let config_path = arguments
        .get(0)
        .expect("Provide the config file path.")
        .to_owned();

    // Load the configuration from the path.
    let config = Config::load(&config_path)?;
    tracing::info!(
        "Successfully loaded the configuration file at {}.",
        config_path,
    );

    let app_state = Arc::new(AppState::new(config));

    // Initialize the secure RPC server.
    initialize_secure_rpc_server(&app_state).await?;

    Ok(())
}

async fn initialize_secure_rpc_server(
    app_state: &AppState, // rpc_client: &RpcClient,
) -> Result<(), Error> {
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
        .register_rpc_method(
            external::SendTransaction::METHOD_NAME,
            external::SendTransaction::handler,
        )?
        .register_rpc_method(
            external::SendRawTransaction::METHOD_NAME,
            external::SendRawTransaction::handler,
        )?
        .register_rpc_method(
            external::SendEncryptedTransaction::METHOD_NAME,
            external::SendEncryptedTransaction::handler,
        )?
        .init(format!("0.0.0.0:{}", app_state.config().secure_rpc_port()?))
        .await?;

    tracing::info!(
        "Successfully started the Secure RPC server: {}",
        format!("0.0.0.0:{}", app_state.config().secure_rpc_port()?)
    );

    let server_handle = tokio::spawn(async move {
        secure_rpc_server.stopped().await;
    });

    server_handle.await.unwrap();

    Ok(())
}
