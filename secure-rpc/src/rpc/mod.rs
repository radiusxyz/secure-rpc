pub mod prelude {
    pub use std::sync::Arc;

    pub use json_rpc::{RpcClient, RpcError, RpcParameter};
    pub use sequencer::types::*;
    pub use serde::{Deserialize, Serialize};

    pub use crate::error::Error;
}

mod decrypt_transaction;
mod encrypt_transaction;
pub mod eth;
mod request_to_send_encrypted_transaction;
mod request_to_send_raw_transaction;

use std::fmt::Debug;

use async_trait::async_trait;
pub use decrypt_transaction::DecryptTransaction;
pub use encrypt_transaction::EncryptTransaction;
use json_rpc::ArrayRpcClient;
pub use request_to_send_encrypted_transaction::*;
pub use request_to_send_raw_transaction::*;
use serde::de::DeserializeOwned;

use crate::{rpc::prelude::*, state::AppState};

// TODO(jaemin): Port to radius sequencer sdk json rpc
#[async_trait]
pub trait ExternalRpcParameter: Clone + Debug + DeserializeOwned + Send + Serialize {
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

pub async fn forward_to_array_rpc_request<T: ExternalRpcParameter>(
    rpc_parameter: T,
    rpc_client: Arc<ArrayRpcClient>,
) -> Result<T::Output, RpcError> {
    rpc_client
        .request(T::METHOD_NAME, rpc_parameter.rpc_method())
        .await
        .map_err(|error| error.into())
}

#[macro_export]
macro_rules! impl_external_array_rpc_forwarder {
    ($method_struct:ident, $method_name:expr, $output_type:ty) => {
        #[async_trait]
        impl ExternalRpcParameter for $method_struct {
            const METHOD_NAME: &'static str = $method_name;

            type Output = $output_type;

            fn rpc_method(&self) -> Self {
                self.clone()
            }

            async fn handler(
                parameter: RpcParameter,
                context: Arc<$crate::state::AppState>,
            ) -> Result<Self::Output, RpcError> {
                let parameter = parameter.parse::<Self>()?;

                let array_rpc_client = context.rollup_rpc_client().rpc_client();

                Ok(forward_to_array_rpc_request(parameter, array_rpc_client).await?)
            }
        }
    };
}
