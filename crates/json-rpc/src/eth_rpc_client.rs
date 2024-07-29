use std::sync::Arc;

use jsonrpsee::{
    core::{client::ClientT, traits::ToRpcParams},
    http_client::{HttpClient, HttpClientBuilder},
};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::{value::RawValue, Value};
use tokio::time::{sleep, Duration};

use crate::{Error, ErrorKind};

pub struct EthRpcClient {
    http_client: Arc<HttpClient>,
    timeout: u64,
    retry: u8,
    retry_interval: u64,
}

unsafe impl Send for EthRpcClient {}

unsafe impl Sync for EthRpcClient {}

impl Clone for EthRpcClient {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            timeout: self.timeout,
            retry: self.retry,
            retry_interval: self.retry_interval,
        }
    }
}

impl EthRpcClient {
    pub const DEFAULT_TIMEOUT: u64 = 5;
    pub const DEFAULT_RETRY: u8 = 0;
    pub const DEFAULT_RETRY_INTERVAL: u64 = 0;

    pub fn new(rpc_url: impl AsRef<str>) -> Result<Self, Error> {
        let http_client = HttpClientBuilder::new()
            .request_timeout(Duration::from_secs(Self::DEFAULT_TIMEOUT))
            .build(rpc_url.as_ref())
            .map_err(|error| (ErrorKind::BuildClient, error))?;

        Ok(Self {
            http_client: Arc::new(http_client),
            timeout: Self::DEFAULT_TIMEOUT,
            retry: Self::DEFAULT_RETRY,
            retry_interval: Self::DEFAULT_RETRY_INTERVAL,
        })
    }

    pub fn timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }

    pub fn max_retry(mut self, value: u8) -> Self {
        self.retry = value;
        self
    }

    /// Retry interval in seconds
    pub fn retry_interval(mut self, value: u64) -> Self {
        self.retry_interval = value;
        self
    }

    async fn request_inner<P, R>(&self, name: &'static str, method: P) -> Result<R, Error>
    where
        P: Clone + Serialize + Send,
        R: DeserializeOwned,
    {
        let method = EthParameter::from(method);
        self.http_client
            .request(name, method)
            .await
            .map_err(|error| (ErrorKind::RpcRequest, error).into())
    }

    pub async fn request<P, R>(&self, name: &'static str, method: P) -> Result<R, Error>
    where
        P: Clone + Serialize + Send,
        R: DeserializeOwned,
    {
        if self.retry != 0 {
            for _ in 0..self.retry {
                if let Some(response) = self.request_inner(name, method.clone()).await.ok() {
                    return Ok(response);
                } else {
                    sleep(Duration::from_secs(self.retry_interval)).await;
                }
            }
        }

        self.request_inner(name, method).await
    }
}

/// Wrapper for the RPC request parameter.
pub(crate) struct EthParameter<P>(P)
where
    P: Clone + Serialize + Send;

impl<P> Clone for EthParameter<P>
where
    P: Clone + Serialize + Send,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P> From<P> for EthParameter<P>
where
    P: Clone + Serialize + Send,
{
    fn from(value: P) -> Self {
        Self(value)
    }
}

impl<P> ToRpcParams for EthParameter<P>
where
    P: Clone + Serialize + Send,
{
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
        let json_value = serde_json::to_value(&self.0)?;

        let json_string = if let Some(json_val) = json_value.as_object() {
            serde_json::to_string(
                &json_val
                    .iter()
                    .map(|(_key, value)| value.clone())
                    .collect::<Vec<Value>>(),
            )?
        } else {
            serde_json::to_string(&json_value)?
        };

        RawValue::from_string(json_string).map(Some)
    }
}
