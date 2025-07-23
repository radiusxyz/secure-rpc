mod config_option;
mod config_path;

use std::fs;

pub use config_option::ConfigOption;
pub use config_path::ConfigPath;
pub use serde::{Deserialize, Serialize};

use super::transaction::EncryptedTransactionType;

pub const DEFAULT_HOME_PATH: &str = ".secure-rpc";
pub const LOG_DIR_NAME: &str = "logs";
pub const CONFIG_FILE_NAME: &str = "Config.toml";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    rollup_id: String,
    external_rpc_url: String,
    external_ws_url: String,
    tx_orderer_rpc_url_list: Vec<String>,
    rollup_rpc_url: String,
    rollup_ws_url: String,
    encrypted_transaction_type: EncryptedTransactionType,
    distributed_key_generation_rpc_url: String,
}

impl Config {
    pub fn load(config_option: &mut ConfigOption) -> Result<Self, ConfigError> {
        let config_path = config_option
            .path
            .get_or_insert_with(|| ConfigPath::default().as_ref().into())
            .clone();

        let config_file_path = config_path.join(CONFIG_FILE_NAME);
        let config_string = fs::read_to_string(config_file_path).map_err(|err| {
            tracing::error!("Failed to load config file: {:?}", err);
            ConfigError::Load(err)
        })?;

        let config_file: ConfigOption = toml::from_str(&config_string).map_err(|err| {
            tracing::error!("Failed to parse config file: {:?}", err);
            ConfigError::Parse(err)
        })?;
        let merged_config_option = config_file.merge(config_option);

        let encrypted_transaction_type = merged_config_option.encrypted_transaction_type.unwrap();
        let tx_orderer_rpc_url_list = merged_config_option
            .tx_orderer_rpc_url_list
            .unwrap()
            .split(',')
            .map(str::trim)
            .map(String::from)
            .collect::<Vec<_>>();

        Ok(Config {
            rollup_id: merged_config_option.rollup_id.unwrap(),
            external_rpc_url: merged_config_option.external_rpc_url.unwrap(),
            external_ws_url: merged_config_option.external_ws_url.unwrap(),
            tx_orderer_rpc_url_list,
            rollup_rpc_url: merged_config_option.rollup_rpc_url.unwrap(),
            rollup_ws_url: merged_config_option.rollup_ws_url.unwrap(),
            encrypted_transaction_type: EncryptedTransactionType::from(encrypted_transaction_type),
            distributed_key_generation_rpc_url: merged_config_option
                .distributed_key_generation_rpc_url
                .unwrap(),
        })
    }

    pub fn rollup_id(&self) -> &str {
        &self.rollup_id
    }

    pub fn external_rpc_url(&self) -> &str {
        &self.external_rpc_url
    }

    pub fn external_rpc_port(&self) -> Result<String, ConfigError> {
        self.external_rpc_url
            .split(':')
            .last()
            .map(String::from)
            .ok_or(ConfigError::InvalidExternalPort)
    }

    pub fn external_ws_url(&self) -> &str {
        &self.external_ws_url
    }

    pub fn external_ws_port(&self) -> Result<String, ConfigError> {
        self.external_ws_url
            .split(':')
            .last()
            .map(String::from)
            .ok_or(ConfigError::InvalidExternalPort)
    }

    pub fn tx_orderer_rpc_url_list(&self) -> &[String] {
        &self.tx_orderer_rpc_url_list
    }

    pub fn rollup_rpc_url(&self) -> &str {
        &self.rollup_rpc_url
    }

    pub fn rollup_ws_url(&self) -> &str {
        &self.rollup_ws_url
    }

    pub fn encrypted_transaction_type(&self) -> &EncryptedTransactionType {
        &self.encrypted_transaction_type
    }

    pub fn distributed_key_generation_rpc_url(&self) -> &str {
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
