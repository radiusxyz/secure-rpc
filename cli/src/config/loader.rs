use std::path::Path;
use anyhow::{Context, Result};
use super::types::Config;

pub fn load_config(config_path: Option<&str>) -> Result<Option<Config>> {
    let path = match config_path {
        Some(path) => path.to_string(),
        None => {
            // Try to find default config.toml in current directory
            let default_path = "config.toml";
            if !Path::new(default_path).exists() {
                return Ok(None);
            }
            default_path.to_string()
        }
    };

    let config_content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", &path))?;

    let config: Config = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse TOML config file: {}", &path))?;

    Ok(Some(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_nonexistent_config() {
        let result = load_config(Some("nonexistent.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_no_default_config() {
        let result = load_config(None);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}