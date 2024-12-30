mod config_option;
mod config_path;

use std::{fs, path::PathBuf};

pub use config_option::ConfigOption;
pub use config_path::ConfigPath;
use sequencer::types::EncryptedTransactionType;
pub use serde::{Deserialize, Serialize};

pub const DEFAULT_HOME_PATH: &str = ".secure-rpc";
pub const LOG_DIR_NAME: &str = "logs";
pub const CONFIG_FILE_NAME: &str = "Config.toml";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    // Rollup ID
    rollup_id: String,

    // External RPC
    external_rpc_url: String,

    // Sequencer
    sequencer_rpc_url: String,

    // Rollup
    rollup_rpc_url: String,

    // encryption
    is_using_encryption: bool,

    // encrypted_transaction_type
    encrypted_transaction_type: EncryptedTransactionType,

    // zkp (when using PVDE)
    is_using_zkp: bool,

    // (when using SKDE)
    distributed_key_generation_rpc_url: String,
}

impl Config {
    pub fn load(config_option: &mut ConfigOption) -> Result<Self, ConfigError> {
        let config_path = match config_option.path.as_mut() {
            Some(config_path) => config_path.clone(),
            None => {
                let config_path: PathBuf = ConfigPath::default().as_ref().into();
                config_option.path = Some(config_path.clone());
                config_path
            }
        };

        // Read config file
        let config_file_path = config_path.join(CONFIG_FILE_NAME);
        let config_string = fs::read_to_string(config_file_path).map_err(ConfigError::Load)?;

        // Parse String to TOML String
        let config_file: ConfigOption =
            toml::from_str(&config_string).map_err(ConfigError::Parse)?;

        // Merge configs from CLI input
        let merged_config_option = config_file.merge(config_option);

        let encrypted_transaction_type = merged_config_option.encrypted_transaction_type.unwrap();

        Ok(Config {
            rollup_id: merged_config_option.rollup_id.unwrap(),
            external_rpc_url: merged_config_option.external_rpc_url.unwrap(),
            sequencer_rpc_url: merged_config_option.sequencer_rpc_url.unwrap(),
            rollup_rpc_url: merged_config_option.rollup_rpc_url.unwrap(),
            is_using_encryption: merged_config_option.is_using_encryption.unwrap(),
            is_using_zkp: merged_config_option.is_using_zkp.unwrap(),
            encrypted_transaction_type: EncryptedTransactionType::from(encrypted_transaction_type),
            distributed_key_generation_rpc_url: merged_config_option
                .distributed_key_generation_rpc_url
                .unwrap(),
        })
    }

    pub fn rollup_id(&self) -> &String {
        &self.rollup_id
    }

    pub fn external_rpc_url(&self) -> &String {
        &self.external_rpc_url
    }

    pub fn external_port(&self) -> Result<String, ConfigError> {
        Ok(self
            .external_rpc_url()
            .split(':')
            .last()
            .ok_or(ConfigError::InvalidExternalPort)?
            .to_string())
    }

    pub fn sequencer_rpc_url(&self) -> &String {
        &self.sequencer_rpc_url
    }

    pub fn rollup_rpc_url(&self) -> &String {
        &self.rollup_rpc_url
    }

    pub fn is_using_encryption(&self) -> bool {
        self.is_using_encryption
    }

    pub fn is_using_zkp(&self) -> bool {
        self.is_using_zkp
    }

    pub fn encrypted_transaction_type(&self) -> &EncryptedTransactionType {
        &self.encrypted_transaction_type
    }

    pub fn distributed_key_generation_rpc_url(&self) -> &String {
        &self.distributed_key_generation_rpc_url
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Load(std::io::Error),
    Parse(toml::de::Error),
    RemoveConfigDirectory(std::io::Error),
    CreateConfigDirectory(std::io::Error),
    CreateConfigFile(std::io::Error),
    CreatePrivateKeyFile(std::io::Error),
    InvalidExternalPort,
    InvalidClusterPort,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ConfigError {}
