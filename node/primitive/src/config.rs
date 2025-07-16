use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub is_dev: bool,
    pub node_name: Option<String>,
    pub external_rpc_url: String,
    pub internal_rpc_url: String,
    pub cluster_rpc_url: String,
    pub tx_orderer_rpc_url: String,
    pub rollup_rpc_url: String,
    pub dkg_rpc_url: String,
    pub rollup_id: String,
    pub db_path: PathBuf,
    pub blockchain_url: String,
    pub contract_address: String,
    pub encrypt_mode: bool,
}

impl NodeConfig {
    pub fn new(
        is_dev: bool,
        node_name: Option<String>,
        external_rpc_url: String,
        internal_rpc_url: String,
        cluster_rpc_url: String,
        tx_orderer_rpc_url: String,
        rollup_rpc_url: String,
        dkg_rpc_url: String,
        rollup_id: String,
        db_path: PathBuf,
        blockchain_url: String,
        contract_address: String,
        encrypt_mode: bool,
    ) -> Self {
        Self {
            is_dev,
            node_name,
            external_rpc_url,
            internal_rpc_url,
            cluster_rpc_url,
            tx_orderer_rpc_url,
            rollup_rpc_url,
            dkg_rpc_url,
            rollup_id,
            db_path,
            blockchain_url,
            contract_address,
            encrypt_mode,
        }
    }

    pub fn is_encrypt_enabled(&self) -> bool {
        self.encrypt_mode
    }
}
