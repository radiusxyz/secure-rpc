use secure_rpc_node_primitive::NodeConfig;

use crate::{node::NodeCommand, Cli, Commands};

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::init();

    match cli.command {
        Commands::Node(command) => run_node_inner(command),
    }
}

fn create_configuration(cli: Box<NodeCommand>) -> NodeConfig {
    NodeConfig::new(
        cli.is_dev,
        cli.node_name,
        cli.secure_rpc_args.rpc_url,
        cli.secure_rpc_args.tx_orderer_rpc_url,
        cli.secure_rpc_args.rollup_rpc_url,
        cli.secure_rpc_args.dkg_rpc_url,
        cli.secure_rpc_args.rollup_id,
        cli.secure_rpc_args.blockchain_url,
        cli.secure_rpc_args.contract_address,
        cli.secure_rpc_args.encrypt_mode,
    )
}

fn run_node_inner(cli: Box<NodeCommand>) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let config = create_configuration(cli);
    // TODO: handle the result
    runtime.block_on(secure_rpc_node_service::run_node(config))?;
    Ok(())
}
