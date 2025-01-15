use crate::rpc::eth::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthNetVersion(Value);

impl RpcParameter<AppState> for EthNetVersion {
    type Response = Value;

    fn method() -> &'static str {
        "net_version"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        super::forward(Self::method(), self, context).await
    }
}
