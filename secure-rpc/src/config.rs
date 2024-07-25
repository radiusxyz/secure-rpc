use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    // Sequencer
    sequencer_rpc_url: String,
    secure_rpc_url: String,
    // Ethereum
    ethereum_rpc_url: String,
    ethereum_websocket_url: String,
    signing_key: String,
    // SSAL
    ssal_contract_address: String,
    cluster_id: String,
    seeder_rpc_url: String,
    // EigenLayer AVS
    delegation_manager_contract_address: String,
    stake_registry_contract_address: String,
    avs_directory_contract_address: String,
    avs_contract_address: String,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let config_string = fs::read_to_string(path).map_err(Error::OpenConfig)?;
        let config: Self = toml::from_str(&config_string).map_err(Error::ParseConfig)?;
        Ok(config)
    }

    /// AD HOC
    pub fn save(mut self, path: impl AsRef<Path>, cluster_id: String) {
        self.cluster_id = cluster_id;
        let config = toml::to_string(&self).unwrap();
        fs::write(path, config).unwrap();
    }

    pub fn sequencer_rpc_url(&self) -> &String {
        &self.sequencer_rpc_url
    }

    pub fn sequencer_port(&self) -> Result<u16, Error> {
        self.sequencer_rpc_url
            .split(':')
            .last()
            .ok_or(Error::InvalidSequencerPort)?
            .parse::<u16>()
            .map_err(|_| Error::InvalidSequencerPort)
    }

    pub fn secure_rpc_url(&self) -> &String {
        &self.secure_rpc_url
    }

    pub fn secure_rpc_port(&self) -> Result<u16, Error> {
        self.secure_rpc_url
            .split(':')
            .last()
            .ok_or(Error::InvalidSecureRpcPort)?
            .parse::<u16>()
            .map_err(|_| Error::InvalidSecureRpcPort)
    }

    pub fn ethereum_rpc_url(&self) -> &String {
        &self.ethereum_rpc_url
    }

    pub fn ethereum_websocket_url(&self) -> &String {
        &self.ethereum_websocket_url
    }

    pub fn signing_key(&self) -> &String {
        &self.signing_key
    }

    pub fn ssal_contract_address(&self) -> &String {
        &self.ssal_contract_address
    }

    pub fn cluster_id(&self) -> &String {
        &self.cluster_id
    }

    pub fn seeder_rpc_url(&self) -> &String {
        &self.seeder_rpc_url
    }

    pub fn delegation_manager_contract_address(&self) -> &String {
        &self.delegation_manager_contract_address
    }

    pub fn stake_registry_contract_address(&self) -> &String {
        &self.stake_registry_contract_address
    }

    pub fn avs_directory_contract_address(&self) -> &String {
        &self.avs_directory_contract_address
    }

    pub fn avs_contract_address(&self) -> &String {
        &self.avs_contract_address
    }

    pub fn avs_contact_address(&self) -> &String {
        &self.avs_contract_address
    }
}
