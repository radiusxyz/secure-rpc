mod array_rpc_client;
mod client;
mod error;
mod server;
pub mod types;

pub use array_rpc_client::ArrayRpcClient;
pub use client::RpcClient;
pub use error::{Error, ErrorKind, RpcError};
pub use server::RpcServer;
