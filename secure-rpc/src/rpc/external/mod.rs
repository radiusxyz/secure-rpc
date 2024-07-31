// mod encrypt_transaction;
pub mod eth;
// mod encrypt_transaction;
// mod send_encrypted_transaciton;
// mod send_raw_transaction;
// mod send_transaction;
// mod encrypt_transaction;
// mod request_to_send_encrypted_transaction;

use std::fmt::Debug;

use async_trait::async_trait;
use json_rpc::ArrayRpcClient;
// pub use send_encrypted_transaciton::SendEncryptedTransaction;
// pub use send_raw_transaction::SendRawTransaction;
// pub use send_transaction::SendTransaction;
use serde::de::DeserializeOwned;

use crate::{rpc::prelude::*, state::AppState};

#[async_trait]
pub trait RollupRpcParameter: Clone + Debug + DeserializeOwned + Send + Serialize {
    const METHOD_NAME: &'static str;

    type Output: Debug + DeserializeOwned + Send + Serialize;

    fn rpc_method(&self) -> Self {
        self.clone()
    }

    async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<Self::Output, RpcError>;
}

pub async fn forward_to_rpc_request<T: RollupRpcParameter>(
    rpc_parameter: T,
    rpc_client: Arc<ArrayRpcClient>,
) -> Result<T::Output, RpcError> {
    rpc_client
        .request(T::METHOD_NAME, rpc_parameter.rpc_method())
        .await
        .map_err(|error| error.into())
}

#[macro_export]
macro_rules! impl_rollup_rpc_forwarder {
    ($method_struct:ident, $method_name:expr, $output_type:ty) => {
        #[async_trait]
        impl RollupRpcParameter for $method_struct {
            const METHOD_NAME: &'static str = $method_name;

            type Output = $output_type;

            fn rpc_method(&self) -> Self {
                self.clone()
            }

            async fn handler(
                parameter: RpcParameter,
                context: Arc<crate::state::AppState>,
            ) -> Result<Self::Output, RpcError> {
                let parameter = parameter.parse::<Self>()?;

                let eth_rpc_client = context.ethereum_rpc_client().rpc_client();

                Ok(forward_to_rpc_request(parameter, eth_rpc_client).await?)
            }
        }
    };
}
