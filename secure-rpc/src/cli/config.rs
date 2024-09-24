use std::{fs, path::PathBuf};

use sequencer::types::EncryptedTransactionType;
use serde::{Deserialize, Serialize};

use super::{ConfigOption, ConfigPath, CONFIG_FILE_NAME};
use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    // Rollup ID
    rollup_id: String,

    // Secure RPC
    secure_rpc_url: String,

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
    key_management_rpc_url: String,
}

impl Config {
    pub fn load(config_option: &mut ConfigOption) -> Result<Self, Error> {
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
        let config_string =
            fs::read_to_string(config_file_path).map_err(|_| Error::LoadConfigOption)?;

        // Parse String to TOML String
        let config_file: ConfigOption =
            toml::from_str(&config_string).map_err(|_| Error::ParseTomlString)?;

        // Merge configs from CLI input
        let merged_config_option = config_file.merge(config_option);

        let encrypted_transaction_type = merged_config_option.encrypted_transaction_type.unwrap();

        Ok(Config {
            rollup_id: merged_config_option.rollup_id.unwrap(),
            secure_rpc_url: merged_config_option.secure_rpc_url.unwrap(),
            sequencer_rpc_url: merged_config_option.sequencer_rpc_url.unwrap(),
            rollup_rpc_url: merged_config_option.rollup_rpc_url.unwrap(),
            is_using_encryption: merged_config_option.is_using_encryption.unwrap(),
            is_using_zkp: merged_config_option.is_using_zkp.unwrap(),
            encrypted_transaction_type: EncryptedTransactionType::from(encrypted_transaction_type),
            key_management_rpc_url: merged_config_option.key_management_system_rpc_url.unwrap(),
        })
    }

    pub fn rollup_id(&self) -> &String {
        &self.rollup_id
    }

    pub fn secure_rpc_url(&self) -> &String {
        &self.secure_rpc_url
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

    pub fn key_management_system_rpc_url(&self) -> &String {
        &self.key_management_rpc_url
    }
}
