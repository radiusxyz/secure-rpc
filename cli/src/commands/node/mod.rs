mod data_dir;
mod rpc_server;
mod secure_rpc;

use data_dir::DataDirArgs;
use rpc_server::RpcServerArgs;
use secure_rpc::SecureRpcArgs;

use crate::Parser;

#[derive(Debug, Parser)]
pub struct NodeCommand {
    #[arg(long = "dev")]
    pub is_dev: bool,
    #[arg(long = "node-name")]
    pub node_name: Option<String>,
    #[command(flatten)]
    pub rpc_server: RpcServerArgs,
    #[command(flatten)]
    pub secure_rpc: SecureRpcArgs,
    #[command(flatten)]
    pub data_dir: DataDirArgs,
}
