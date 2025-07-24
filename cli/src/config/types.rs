use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub node: Option<NodeConfig>,
    pub secure_rpc: Option<SecureRpcConfig>,
    pub operator: Option<OperatorConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeConfig {
    pub is_dev: Option<bool>,
    pub node_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecureRpcConfig {
    pub rollup_id: Option<String>,
    pub rpc_url: Option<String>,
    pub encrypt_mode: Option<bool>,
    pub tx_orderer_rpc_url: Option<String>,
    pub rollup_rpc_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OperatorConfig {
    pub blockchain: Option<BlockchainOperatorConfig>,
    pub basic: Option<BasicOperatorConfig>,
}

#[derive(Debug, Clone, Deserialize)]  
pub struct BlockchainOperatorConfig {
    pub blockchain_http_rpc_url: Option<String>,
    pub blockchain_ws_rpc_url: Option<String>,
    pub contract_address: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BasicOperatorConfig {
    pub dkg_rpc_urls: Option<Vec<String>>,
}