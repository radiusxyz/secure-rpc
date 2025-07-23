use std::{fs, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::{ConfigPath, CONFIG_FILE_NAME};
use crate::error::Error;

const DEFAULT_EXTERNAL_RPC_URL: &str = "http://127.0.0.1:9000";
const DEFAULT_EXTERNAL_WS_URL: &str = "ws://127.0.0.1:9111";
const DEFAULT_TX_ORDERER_RPC_URL_LIST: &str = "http://127.0.0.1:3000";
const DEFAULT_ROLLUP_RPC_URL: &str = "http://127.0.0.1:8123";
const DEFAULT_ROLLUP_WS_URL: &str = "ws://127.0.0.1:8123";
const DEFAULT_ENCRYPTED_TRANSACTION_TYPE: &str = "skde";
const DEFAULT_DISTRIBUTED_KEY_GENERATION_RPC_URL: &str = "http://127.0.0.1:7100";

#[derive(Debug, Deserialize, Parser, Serialize)]
pub struct ConfigOption {
    #[doc = "Set the configuration file path to load from"]
    #[clap(long = "path")]
    pub path: Option<PathBuf>,

    #[doc = "Set rollup id"]
    #[clap(long = "rollup-id")]
    pub rollup_id: Option<String>,

    #[doc = "Set the external rpc url"]
    #[clap(long = "external-rpc-url")]
    pub external_rpc_url: Option<String>,

    #[doc = "Set the external ws url"]
    #[clap(long = "external-ws-url")]
    pub external_ws_url: Option<String>,

    #[doc = "Set the tx orderer rpc url list"]
    #[clap(long = "tx-orderer-rpc-url-list")]
    pub tx_orderer_rpc_url_list: Option<String>,

    #[doc = "Set the rollup rpc url"]
    #[clap(long = "rollup-rpc-url")]
    pub rollup_rpc_url: Option<String>,

    #[doc = "Set the rollup websocket url"]
    #[clap(long = "rollup-websocket-url")]
    pub rollup_ws_url: Option<String>,

    #[doc = "Set encrypted transaction type"]
    #[clap(long = "encrypted-transaction-type")]
    pub encrypted_transaction_type: Option<String>,

    #[doc = "Set the distributed key generation rpc url"]
    #[clap(long = "distributed-key-generation-rpc-url")]
    pub distributed_key_generation_rpc_url: Option<String>,
}

impl Default for ConfigOption {
    fn default() -> Self {
        Self {
            path: Some(ConfigPath::default().as_ref().into()),
            rollup_id: Some("0".into()),
            external_rpc_url: Some(DEFAULT_EXTERNAL_RPC_URL.into()),
            external_ws_url: Some(DEFAULT_EXTERNAL_WS_URL.into()),
            tx_orderer_rpc_url_list: Some(DEFAULT_TX_ORDERER_RPC_URL_LIST.into()),
            rollup_rpc_url: Some(DEFAULT_ROLLUP_RPC_URL.into()),
            rollup_ws_url: Some(DEFAULT_ROLLUP_WS_URL.into()),
            encrypted_transaction_type: Some(DEFAULT_ENCRYPTED_TRANSACTION_TYPE.into()),
            distributed_key_generation_rpc_url: Some(
                DEFAULT_DISTRIBUTED_KEY_GENERATION_RPC_URL.into(),
            ),
        }
    }
}

impl ConfigOption {
    pub fn load_config(config_option: &mut ConfigOption) -> Result<Self, Error> {
        let config_path = config_option.path.clone().unwrap_or_else(|| {
            let default_path: PathBuf = ConfigPath::default().as_ref().into();
            config_option.path = Some(default_path.clone());
            default_path
        });

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

        let fields = [
            ("Set rollup id", "rollup_id", &self.rollup_id),
            (
                "Set external rpc url",
                "external_rpc_url",
                &self.external_rpc_url,
            ),
            (
                "Set external ws url",
                "external_ws_url",
                &self.external_ws_url,
            ),
            (
                "Set tx orderer rpc url list",
                "tx_orderer_rpc_url_list",
                &self.tx_orderer_rpc_url_list,
            ),
            ("Set rollup rpc url", "rollup_rpc_url", &self.rollup_rpc_url),
            (
                "Set rollup websocket url",
                "rollup_ws_url",
                &self.rollup_ws_url,
            ),
            (
                "Set encrypted transaction type",
                "encrypted_transaction_type",
                &self.encrypted_transaction_type,
            ),
            (
                "Set distributed key generation rpc url",
                "distributed_key_generation_rpc_url",
                &self.distributed_key_generation_rpc_url,
            ),
        ];

        for (comment, name, value) in &fields {
            set_toml_comment(&mut toml_string, comment);
            set_toml_name_value(&mut toml_string, name, value);
        }

        toml_string
    }

    pub fn merge(mut self, other: &ConfigOption) -> Self {
        macro_rules! merge_field {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field.clone_from(&other.$field);
                }
            };
        }

        merge_field!(path);
        merge_field!(rollup_id);
        merge_field!(external_rpc_url);
        merge_field!(external_ws_url);
        merge_field!(tx_orderer_rpc_url_list);
        merge_field!(rollup_rpc_url);
        merge_field!(rollup_ws_url);
        merge_field!(encrypted_transaction_type);
        merge_field!(distributed_key_generation_rpc_url);

        self
    }
}

fn set_toml_comment(toml_string: &mut String, comment: &str) {
    toml_string.push_str(&format!("# {}\n", comment));
}

fn set_toml_name_value<T>(toml_string: &mut String, name: &str, value: &Option<T>)
where
    T: std::fmt::Debug,
{
    let name_value = match value {
        Some(value) => format!("{} = {:?}\n\n", name, value),
        None => format!("# {} = {:?}\n\n", name, value),
    };

    toml_string.push_str(&name_value);
}
