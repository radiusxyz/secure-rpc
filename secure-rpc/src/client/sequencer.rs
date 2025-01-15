use std::sync::Arc;

use json_rpc::RpcClient;
use radius_sdk::json_rpc::client::{Id, RpcClientError};
use sequencer::{
    rpc::external::{SendEncryptedTransaction, SendRawTransaction},
    types::{EncryptedTransaction, OrderCommitment, RawTransaction},
};

pub struct SequencerRpcClient {
    inner: Arc<SequencerRpcClientInner>,
}

struct SequencerRpcClientInner {
    rpc_url: String,
    rpc_client: RpcClient,
}

impl Clone for SequencerRpcClient {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl SequencerRpcClient {
    pub fn new(sequencer_rpc_url: impl AsRef<str>) -> Result<Self, RpcClientError> {
        let inner = SequencerRpcClientInner {
            rpc_url: sequencer_rpc_url.as_ref().to_owned(),
            rpc_client: RpcClient::new()?,
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub async fn send_encrypted_transaction(
        &self,
        rollup_id: String,
        encrypted_transaction: EncryptedTransaction,
    ) -> Result<OrderCommitment, RpcClientError> {
        let parameter = SendEncryptedTransaction {
            rollup_id,
            encrypted_transaction,
        };

        tracing::info!("Send encrypted transaction: {:?}", parameter);

        self.inner
            .rpc_client
            .request(
                &self.inner.rpc_url,
                "send_encrypted_transaction",
                &parameter,
                Id::Null,
            )
            .await
    }

    pub async fn send_raw_transaction(
        &self,
        rollup_id: String,
        raw_transaction: RawTransaction,
    ) -> Result<OrderCommitment, RpcClientError> {
        let parameter = SendRawTransaction {
            rollup_id,
            raw_transaction,
        };

        tracing::info!("Send raw transaction: {:?}", parameter);

        self.inner
            .rpc_client
            .request(
                &self.inner.rpc_url,
                "send_raw_transaction",
                &parameter,
                Id::Null,
            )
            .await
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.inner.rpc_client
    }
}
