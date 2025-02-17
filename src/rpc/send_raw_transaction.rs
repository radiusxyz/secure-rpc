use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendRawTransaction {
    pub rollup_id: String,
    pub raw_transaction: RawTransaction,
}

impl RpcParameter<AppState> for SendRawTransaction {
    type Response = OrderCommitment;

    fn method() -> &'static str {
        "send_raw_transaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        let seed: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        match context
            .rpc_client()
            .request(
                context
                    .config()
                    .tx_orderer_rpc_url_list()
                    .choose(&mut StdRng::seed_from_u64(seed))
                    .ok_or(Error::EmptyTxOrdererRpcUrl)?,
                Self::method(),
                self,
                Id::Null,
            )
            .await
        {
            Ok(order_commitment) => {
                tracing::info!("Order commitment: {:?}", order_commitment);
                Ok(order_commitment)
            }
            Err(error) => {
                tracing::error!("Failed to send raw transaction: {:?}", error);
                Err(error.into())
            }
        }
    }
}
