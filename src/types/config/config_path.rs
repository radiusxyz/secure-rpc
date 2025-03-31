use std::{
    env, fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    types::config::{ConfigOption, CONFIG_FILE_NAME},
};

#[derive(Debug, Deserialize, Parser, Serialize)]
pub struct ConfigPath {
    #[doc = "Set the secure-rpc configuration path"]
    #[clap(long = "path", default_value_t = Self::default().to_string())]
    path: String,
}

impl std::fmt::Display for ConfigPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl AsRef<Path> for ConfigPath {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl Default for ConfigPath {
    fn default() -> Self {
        let home_path = env::var("HOME").unwrap_or_else(|err| {
            tracing::warn!("Failed to get HOME environment variable: {:?}", err);
            ".".to_string()
        });
        let path = PathBuf::from(home_path)
            .join(super::DEFAULT_HOME_PATH)
            .to_string_lossy()
            .into_owned();

        Self { path }
    }
}

impl ConfigPath {
    pub fn init(&self) -> Result<(), Error> {
        let path = self.as_ref();

        if path.exists() {
            fs::remove_dir_all(path).map_err(|err| {
                tracing::error!("Failed to remove config directory: {:?}", err);
                Error::RemoveConfigDirectory
            })?;
        }

        fs::create_dir_all(path).map_err(|err| {
            tracing::error!("Failed to create config directory: {:?}", err);
            Error::CreateConfigDirectory
        })?;

        let config_file_path = path.join(CONFIG_FILE_NAME);
        let config_toml_string = ConfigOption::default().get_toml_string();
        fs::write(config_file_path, config_toml_string).map_err(|err| {
            tracing::error!("Failed to create config file: {:?}", err);
            Error::CreateConfigFile
        })?;

        tracing::info!("Created a new config directory at {:?}", path);
        Ok(())
    }
}
