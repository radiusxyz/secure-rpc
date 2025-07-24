use secure_rpc_node_primitive::NodeConfig;

use crate::{node::NodeCommand, Cli, Commands, commands::node::{OperatorService, BlockchainOperatorArgs, BasicOperatorArgs}, config::{load_config, Config}};

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::init();

    // Load configuration file
    let config = load_config(cli.config.as_deref())?;

    match cli.command {
        Commands::Node(command) => run_node_inner(command, config),
    }
}

fn create_configuration(cli: &NodeCommand, config: Option<Config>) -> NodeConfig {
    use secure_rpc_node_primitive::constants::rpc_url::{
        DEFAULT_ROLLUP_RPC_URL, 
        DEFAULT_TX_ORDERER_RPC_URL_LIST,
        DEFAULT_RPC_URL
    };
    
    // Merge config file values with CLI arguments (CLI takes precedence)
    let _config_node = config.as_ref().and_then(|c| c.node.as_ref());
    let config_secure_rpc = config.as_ref().and_then(|c| c.secure_rpc.as_ref());

    // For node settings - CLI values always take precedence for now
    let is_dev = cli.is_dev;
    let node_name = cli.node_name.clone();
    
    // For secure RPC settings - use config values if CLI has default values
    let rpc_url = if cli.secure_rpc_args.rpc_url == DEFAULT_RPC_URL {
        config_secure_rpc.and_then(|c| c.rpc_url.clone()).unwrap_or_else(|| cli.secure_rpc_args.rpc_url.clone())
    } else {
        cli.secure_rpc_args.rpc_url.clone()
    };

    let tx_orderer_rpc_url = if cli.secure_rpc_args.tx_orderer_rpc_url == DEFAULT_TX_ORDERER_RPC_URL_LIST {
        config_secure_rpc.and_then(|c| c.tx_orderer_rpc_url.clone()).unwrap_or_else(|| cli.secure_rpc_args.tx_orderer_rpc_url.clone())
    } else {
        cli.secure_rpc_args.tx_orderer_rpc_url.clone()
    };

    let rollup_rpc_url = if cli.secure_rpc_args.rollup_rpc_url == DEFAULT_ROLLUP_RPC_URL {
        config_secure_rpc.and_then(|c| c.rollup_rpc_url.clone()).unwrap_or_else(|| cli.secure_rpc_args.rollup_rpc_url.clone())
    } else {
        cli.secure_rpc_args.rollup_rpc_url.clone()
    };

    let rollup_id = if cli.secure_rpc_args.rollup_id == "0" {
        config_secure_rpc.and_then(|c| c.rollup_id.clone()).unwrap_or_else(|| cli.secure_rpc_args.rollup_id.clone())
    } else {
        cli.secure_rpc_args.rollup_id.clone()
    };

    let encrypt_mode = if cli.secure_rpc_args.encrypt_mode == true {
        config_secure_rpc.and_then(|c| c.encrypt_mode).unwrap_or(cli.secure_rpc_args.encrypt_mode)
    } else {
        cli.secure_rpc_args.encrypt_mode
    };

    NodeConfig::new(
        is_dev,
        node_name,
        rpc_url,
        tx_orderer_rpc_url,
        rollup_rpc_url,
        rollup_id,
        encrypt_mode,
    )
}

fn run_node_inner(cli: Box<NodeCommand>, config: Option<Config>) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let node_config = create_configuration(&cli, config.clone());
    match cli.operator_service {
        OperatorService::Blockchain(args) => {
            let (blockchain_http_rpc_url, blockchain_ws_rpc_url, contract_address) = 
                merge_blockchain_operator_args(args, config.as_ref());
            runtime.block_on(secure_rpc_node_service::run_blockchain_operator_secure_rpc_node(node_config, blockchain_http_rpc_url, blockchain_ws_rpc_url, contract_address))?;
        }
        OperatorService::Basic(args) => {
            let dkg_rpc_urls = merge_basic_operator_args(args, config.as_ref());
            runtime.block_on(secure_rpc_node_service::run_basic_operator_secure_rpc_node(node_config, dkg_rpc_urls))?;
        }
    }
    Ok(())
}

fn merge_blockchain_operator_args(
    args: BlockchainOperatorArgs, 
    config: Option<&Config>
) -> (String, String, String) {
    use secure_rpc_node_primitive::constants::rpc_url::{
        DEFAULT_BLOCKCHAIN_URL,
        DEFAULT_CONTRACT_ADDRESS,
    };

    let config_blockchain = config.and_then(|c| c.operator.as_ref())
        .and_then(|o| o.blockchain.as_ref());

    let blockchain_http_rpc_url = if args.blockchain_http_rpc_url == DEFAULT_BLOCKCHAIN_URL {
        config_blockchain.and_then(|c| c.blockchain_http_rpc_url.clone()).unwrap_or(args.blockchain_http_rpc_url)
    } else {
        args.blockchain_http_rpc_url
    };

    let blockchain_ws_rpc_url = if args.blockchain_ws_rpc_url == DEFAULT_BLOCKCHAIN_URL {
        config_blockchain.and_then(|c| c.blockchain_ws_rpc_url.clone()).unwrap_or(args.blockchain_ws_rpc_url)
    } else {
        args.blockchain_ws_rpc_url
    };

    let contract_address = if args.contract_address == DEFAULT_CONTRACT_ADDRESS {
        config_blockchain.and_then(|c| c.contract_address.clone()).unwrap_or(args.contract_address)
    } else {
        args.contract_address
    };

    (blockchain_http_rpc_url, blockchain_ws_rpc_url, contract_address)
}

fn merge_basic_operator_args(
    args: BasicOperatorArgs, 
    config: Option<&Config>
) -> Vec<String> {
    use secure_rpc_node_primitive::constants::rpc_url::DEFAULT_DKG_RPC_URL;

    let config_basic = config.and_then(|c| c.operator.as_ref())
        .and_then(|o| o.basic.as_ref());

    // Check if CLI has default values (single URL matching DEFAULT_DKG_RPC_URL)
    if args.dkg_rpc_urls.len() == 1 && args.dkg_rpc_urls[0] == DEFAULT_DKG_RPC_URL {
        config_basic.and_then(|c| c.dkg_rpc_urls.clone()).unwrap_or(args.dkg_rpc_urls)
    } else {
        args.dkg_rpc_urls
    }
}
