// TODO: This crate have to port to radius sequencer sdk json rpc
mod array_rpc_client;
mod error;

pub use array_rpc_client::ArrayRpcClient;
pub use error::{Error, ErrorKind};
pub use jsonrpsee::types::Params;
pub use radius_sdk::json_rpc::{
    client::RpcClient,
    server::{RpcParameter, RpcServer, RpcServerError},
};
