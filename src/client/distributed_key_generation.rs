use std::sync::Arc;

use radius_sdk::json_rpc::client::{Id, RpcClient, RpcClientError};
use serde::{Deserialize, Serialize};

pub struct DistributedKeyGenerationClient {
    inner: Arc<DistributedKeyGenerationClientInner>,
}

struct DistributedKeyGenerationClientInner {
    rpc_url: String,
    rpc_client: RpcClient,
}

impl Clone for DistributedKeyGenerationClient {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl DistributedKeyGenerationClient {
    pub fn new(rpc_url: impl AsRef<str>) -> Result<Self, RpcClientError> {
        let inner = DistributedKeyGenerationClientInner {
            rpc_url: rpc_url.as_ref().to_owned(),
            rpc_client: RpcClient::new()?,
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub async fn get_latest_encryption_key(
        &self,
    ) -> Result<GetLatestEncryptionKeyReturn, RpcClientError> {
        let parameter = GetLatestEncryptionKey {};

        self.inner
            .rpc_client
            .request(
                &self.inner.rpc_url,
                GetLatestEncryptionKey::METHOD_NAME,
                &parameter,
                Id::Null,
            )
            .await
    }

    pub async fn get_encryption_key(
        &self,
        key_id: u64,
    ) -> Result<GetEncryptionKeyReturn, RpcClientError> {
        let parameter = GetEncryptionKey { key_id };

        self.inner
            .rpc_client
            .request(
                &self.inner.rpc_url,
                GetEncryptionKey::METHOD_NAME,
                &parameter,
                Id::Null,
            )
            .await
    }

    pub async fn get_decryption_key(
        &self,
        key_id: u64,
    ) -> Result<GetDecryptionKeyResponse, RpcClientError> {
        let parameter = GetDecryptionKey { key_id };

        self.inner
            .rpc_client
            .request(
                &self.inner.rpc_url,
                GetDecryptionKey::METHOD_NAME,
                &parameter,
                Id::Null,
            )
            .await
    }

    pub async fn get_skde_params(&self) -> Result<GetSkdeParamsResponse, RpcClientError> {
        let parameter = GetSkdeParams {};

        self.inner
            .rpc_client
            .request(
                &self.inner.rpc_url,
                GetSkdeParams::METHOD_NAME,
                &parameter,
                Id::Null,
            )
            .await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLatestEncryptionKey {}

impl GetLatestEncryptionKey {
    pub const METHOD_NAME: &'static str = "get_latest_encryption_key";
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLatestEncryptionKeyReturn {
    pub encryption_key: String,
    pub key_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetEncryptionKey {
    pub key_id: u64,
}

impl GetEncryptionKey {
    pub const METHOD_NAME: &'static str = "get_encryption_key";
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetEncryptionKeyReturn {
    pub encryption_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetDecryptionKey {
    pub key_id: u64,
}

impl GetDecryptionKey {
    pub const METHOD_NAME: &'static str = "get_decryption_key";
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetDecryptionKeyResponse {
    pub decryption_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSkdeParams {}

impl GetSkdeParams {
    pub const METHOD_NAME: &'static str = "get_skde_params";
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSkdeParamsResponse {
    pub skde_params: skde::delay_encryption::SkdeParams,
}
