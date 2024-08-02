use async_trait::async_trait;

use crate::{
    impl_external_array_rpc_forwarder,
    rpc::{
        {forward_to_array_rpc_request, ExternalRpcParameter},
        prelude::*,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthNetVersion {}

impl_external_array_rpc_forwarder!(EthNetVersion, "net_version", String);
