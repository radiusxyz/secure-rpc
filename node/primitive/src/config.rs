

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub is_dev: bool,
    pub node_name: String,
    pub rpc_url: String,
    pub tx_orderer_rpc_url: String,
    pub rollup_rpc_url: String,
    pub rollup_id: String,
    pub encrypt_mode: bool,
}

impl NodeConfig {
    pub fn new(
        is_dev: bool,
        node_name: String,
        rpc_url: String,
        tx_orderer_rpc_url: String,
        rollup_rpc_url: String,
        rollup_id: String,
        encrypt_mode: bool,
    ) -> Self {
        Self {
            is_dev,
            node_name,
            rpc_url,
            tx_orderer_rpc_url,
            rollup_rpc_url,
            rollup_id,
            encrypt_mode,
        }
    }

    pub fn log_config(&self) {
        tracing::info!("=== Node Configuration ===");
        tracing::info!("📛 Node Name: {}", self.node_name);
        tracing::info!("🔧 Development Mode: {}", self.is_dev);
        tracing::info!("🔐 Encryption Mode: {}", self.encrypt_mode);
        tracing::info!("📡 RPC URL: {}", self.rpc_url);
        tracing::info!("📋 TX Orderer RPC URL: {}", self.tx_orderer_rpc_url);
        tracing::info!("🔄 Rollup RPC URL: {}", self.rollup_rpc_url);
        tracing::info!("🆔 Rollup ID: {}", self.rollup_id);
    }

    pub fn is_encrypt_enabled(&self) -> bool {
        self.encrypt_mode
    }
}
