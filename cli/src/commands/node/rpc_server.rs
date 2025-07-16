use std::net::{IpAddr, Ipv4Addr};

use crate::Args;

#[derive(Debug, Args)]
pub struct RpcServerArgs {
    #[arg(long = "internal.addr", default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub internal_rpc_url: IpAddr,
    #[arg(long = "internal.port", default_value_t = 3000)]
    pub internal_rpc_port: u16,
    #[arg(long = "external.addr", default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub external_rpc_url: IpAddr,
    #[arg(long = "external.port", default_value_t = 3001)]
    pub external_rpc_port: u16,
    #[arg(long = "cluster.addr", default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub cluster_rpc_url: IpAddr,
    #[arg(long = "cluster.port", default_value_t = 3002)]
    pub cluster_rpc_port: u16,
}

impl Default for RpcServerArgs {
    fn default() -> Self {
        Self {
            internal_rpc_url: IpAddr::V4(Ipv4Addr::LOCALHOST).into(),
            internal_rpc_port: 3000,
            external_rpc_url: IpAddr::V4(Ipv4Addr::LOCALHOST).into(),
            external_rpc_port: 3001,
            cluster_rpc_url: IpAddr::V4(Ipv4Addr::LOCALHOST).into(),
            cluster_rpc_port: 3002,
        }
    }
}

impl RpcServerArgs {
    pub fn external_rpc_url(&self) -> String {
        format!(
            "http://{}:{}",
            self.external_rpc_url, self.external_rpc_port
        )
    }

    pub fn internal_rpc_url(&self) -> String {
        format!(
            "http://{}:{}",
            self.internal_rpc_url, self.internal_rpc_port
        )
    }

    pub fn cluster_rpc_url(&self) -> String {
        format!("http://{}:{}", self.cluster_rpc_url, self.cluster_rpc_port)
    }
}
