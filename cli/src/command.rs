use std::{path::PathBuf, time::Duration};

use secure_rpc_node_primitive::NodeConfig;

use crate::{node::NodeCommand, Cli, Commands};

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::init();

    match cli.command {
        Commands::Node(command) => run_node_inner(command),
    }
}

fn create_configuration(cli: Box<NodeCommand>) -> NodeConfig {
    let db_path = cli.data_dir.db_path.unwrap();
    NodeConfig::new(
        cli.is_dev,
        cli.node_name,
        cli.rpc_server.external_rpc_url(),
        cli.rpc_server.internal_rpc_url(),
        cli.rpc_server.cluster_rpc_url(),
        cli.secure_rpc.tx_orderer_rpc_url,
        cli.secure_rpc.rollup_rpc_url,
        cli.secure_rpc.dkg_rpc_url,
        cli.secure_rpc.rollup_id,
        db_path,
        cli.secure_rpc.blockchain_url,
        cli.secure_rpc.contract_address,
        cli.secure_rpc.encrypt_mode,
    )
}

fn run_node_inner(cli: Box<NodeCommand>) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let config = create_configuration(cli);
    // TODO: handle the result
    runtime.block_on(secure_rpc_node_service::run_node(config))
}
