use std::{fs, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::{ConfigPath, CONFIG_FILE_NAME};
use crate::error::Error;

const DEFAULT_SECURE_RPC_URL: &str = "http://127.0.0.1:9000";
const DEFAULT_SEQUENCER_RPC_URL: &str = "http://127.0.0.1:3000";
const DEFAULT_ROLLUP_RPC_URL: &str = "http://127.0.0.1:8123";

const DEFAULT_ENCRYPTED_TRANSACTION_TYPE: &str = "skde";
const DEFAULT_KEY_MANAGEMENT_SYSTEM_RPC_URL: &str = "http://127.0.0.1:7100";

#[derive(Debug, Deserialize, Parser, Serialize)]
pub struct ConfigOption {
    #[doc = "Set the configuration file path to load from"]
    #[clap(long = "path")]
    pub path: Option<PathBuf>,

    #[doc = "Set rollup id"]
    #[clap(long = "rollup-id")]
    pub rollup_id: Option<String>,

    #[doc = "Set the secure rpc url"]
    #[clap(long = "secure-rpc-url")]
    pub secure_rpc_url: Option<String>,

    #[doc = "Set the sequencer rpc url"]
    #[clap(long = "sequencer-rpc-url")]
    pub sequencer_rpc_url: Option<String>,

    #[doc = "Set the rollup rpc url"]
    #[clap(long = "rollup-rpc-url")]
    pub rollup_rpc_url: Option<String>,

    #[doc = "Set using encryption"]
    #[clap(long = "is-using-encryption")]
    pub is_using_encryption: Option<bool>,

    #[doc = "Set using zkp"]
    #[clap(long = "is-using-zkp")]
    pub is_using_zkp: Option<bool>,

    #[doc = "Set encrypted transaction type"]
    #[clap(long = "encrypted-transaction-type")]
    pub encrypted_transaction_type: Option<String>,

    #[doc = "Set the key management system rpc url"]
    #[clap(long = "key-management-system-rpc-url")]
    pub key_management_system_rpc_url: Option<String>,
}

impl Default for ConfigOption {
    fn default() -> Self {
        Self {
            path: Some(ConfigPath::default().as_ref().into()),
            rollup_id: Some("0".into()),
            secure_rpc_url: Some(DEFAULT_SECURE_RPC_URL.into()),
            sequencer_rpc_url: Some(DEFAULT_SEQUENCER_RPC_URL.into()),
            rollup_rpc_url: Some(DEFAULT_ROLLUP_RPC_URL.into()),
            is_using_encryption: Some(true),
            is_using_zkp: Some(false),
            encrypted_transaction_type: Some(DEFAULT_ENCRYPTED_TRANSACTION_TYPE.into()),
            key_management_system_rpc_url: Some(DEFAULT_KEY_MANAGEMENT_SYSTEM_RPC_URL.into()),
        }
    }
}

impl ConfigOption {
    pub fn load_config(config_option: &mut ConfigOption) -> Result<Self, Error> {
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
        let config_file: Self =
            toml::from_str(&config_string).map_err(|_| Error::ParseTomlString)?;

        // Merge configs from CLI input
        Ok(config_file.merge(config_option))
    }

    pub fn get_toml_string(&self) -> String {
        let mut toml_string = String::new();

        set_toml_comment(&mut toml_string, "Set rollup id");
        set_toml_name_value(&mut toml_string, "rollup_id", &self.rollup_id);

        set_toml_comment(&mut toml_string, "Set secure rpc url");
        set_toml_name_value(&mut toml_string, "secure_rpc_url", &self.secure_rpc_url);

        set_toml_comment(&mut toml_string, "Set sequencer rpc url");
        set_toml_name_value(
            &mut toml_string,
            "sequencer_rpc_url",
            &self.sequencer_rpc_url,
        );

        set_toml_comment(&mut toml_string, "Set rollup rpc url");
        set_toml_name_value(&mut toml_string, "rollup_rpc_url", &self.rollup_rpc_url);

        set_toml_comment(&mut toml_string, "Set using encryption");
        set_toml_name_value(
            &mut toml_string,
            "is_using_encryption",
            &self.is_using_encryption,
        );

        set_toml_comment(&mut toml_string, "Set using zkp");
        set_toml_name_value(&mut toml_string, "is_using_zkp", &self.is_using_zkp);

        set_toml_comment(&mut toml_string, "Set encrypted transaction type");
        set_toml_name_value(
            &mut toml_string,
            "encrypted_transaction_type",
            &self.encrypted_transaction_type,
        );

        set_toml_comment(&mut toml_string, "Set key management system rpc url");
        set_toml_name_value(
            &mut toml_string,
            "key_management_system_rpc_url",
            &self.key_management_system_rpc_url,
        );

        toml_string
    }

    pub fn merge(mut self, other: &ConfigOption) -> Self {
        if other.path.is_some() {
            self.path.clone_from(&other.path);
        }

        if other.rollup_id.is_some() {
            self.rollup_id.clone_from(&other.rollup_id);
        }

        if other.secure_rpc_url.is_some() {
            self.secure_rpc_url.clone_from(&other.secure_rpc_url);
        }

        if other.sequencer_rpc_url.is_some() {
            self.sequencer_rpc_url.clone_from(&other.sequencer_rpc_url);
        }

        if other.rollup_rpc_url.is_some() {
            self.rollup_rpc_url.clone_from(&other.rollup_rpc_url);
        }

        if other.is_using_encryption.is_some() {
            self.is_using_encryption
                .clone_from(&other.is_using_encryption);
        }

        if other.is_using_zkp.is_some() {
            self.is_using_zkp.clone_from(&other.is_using_zkp);
        }

        if other.encrypted_transaction_type.is_some() {
            self.encrypted_transaction_type
                .clone_from(&other.encrypted_transaction_type);
        }

        if other.key_management_system_rpc_url.is_some() {
            self.key_management_system_rpc_url
                .clone_from(&other.key_management_system_rpc_url);
        }

        self
    }
}

fn set_toml_comment(toml_string: &mut String, comment: &'static str) {
    let comment = format!("# {}\n", comment);

    toml_string.push_str(&comment);
}

fn set_toml_name_value<T>(toml_string: &mut String, name: &'static str, value: &Option<T>)
where
    T: std::fmt::Debug,
{
    let name_value = match value {
        Some(value) => format!("{} = {:?}\n\n", name, value),
        None => format!("# {} = {:?}\n\n", name, value),
    };

    toml_string.push_str(&name_value);
}
