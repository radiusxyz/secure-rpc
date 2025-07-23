
mod secure_rpc;

use uuid::Uuid;
use secure_rpc::SecureRpcArgs;

use crate::Parser;

#[derive(Debug, Parser)]
pub struct NodeCommand {
    #[doc = "Run in development mode"]
    #[arg(long = "dev", default_value_t = true)]
    pub is_dev: bool,
    #[doc = "Name of the node to specify on the network"]
    #[arg(long = "node-name", default_value_t = format!("secure-rpc-provider-{}", Uuid::new_v4()))]
    pub node_name: String,
    #[doc = "Secure RPC configuration"]
    #[command(flatten)]
    pub secure_rpc_args: SecureRpcArgs,
}
