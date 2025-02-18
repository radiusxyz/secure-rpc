use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde_json::Value;

use crate::rpc::{
    prelude::*, send_encrypted_transaction::SendEncryptedTransactionRequest, EncryptTransaction,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(Vec<String>);

// #[derive(Debug, Serialize)]
// struct RawTransactionRequest<'a> {
//     pub rollup_id: &'a str,
//     pub raw_transaction: RawTransactionRequestData<'a>,
// }

// #[derive(Debug, Serialize)]
// struct RawTransactionRequestData<'a> {
//     #[serde(rename = "type")]
//     transaction_type: &'a str,
//     data: &'a str,
// }

impl RpcParameter<AppState> for EthSendRawTransaction {
    type Response = Value;

    fn method() -> &'static str {
        "eth_sendRawTransaction"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        if self.0.is_empty() {
            return Err(Error::EmptyRawTransaction.into());
        }

        let raw_transaction_string = self.0.get(0).unwrap();
        let eth_raw_transaction =
            RawTransaction::from(EthRawTransaction(raw_transaction_string.clone()));

        tracing::info!("encrypt_transaction_params: {:?}", eth_raw_transaction);
        let encrypt_transaction_request = EncryptTransaction {
            raw_transaction: eth_raw_transaction,
        };
        let encrypt_transaction_response =
            encrypt_transaction_request.handler(context.clone()).await?;

        let parameter = SendEncryptedTransactionRequest {
            rollup_id: context.config().rollup_id().to_owned(),
            encrypted_transaction: encrypt_transaction_response.encrypted_transaction,
        };

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
                "send_encrypted_transaction",
                parameter,
                Id::Null,
            )
            .await
        {
            Ok(order_commitment) => {
                tracing::info!("Order commitment: {:?}", order_commitment);
                Ok(order_commitment)
            }
            Err(error) => {
                tracing::error!("Failed to send encrypted transaction: {:?}", error);
                Err(error.into())
            }
        }

        // let raw_transaction_string = self.0.get(0).unwrap();
        // let eth_raw_transaction = EthRawTransaction(raw_transaction_string.clone());
        // let raw_transaction_hash = eth_raw_transaction.raw_transaction_hash();

        // let parameter = RawTransactionRequest {
        //     rollup_id: context.config().rollup_id(),
        //     raw_transaction: RawTransactionRequestData {
        //         transaction_type: "eth",
        //         data: raw_transaction_string,
        //     },
        // };

        // let seed: u64 = std::time::SystemTime::now()
        //     .duration_since(std::time::UNIX_EPOCH)
        //     .unwrap()
        //     .as_nanos()
        //     .try_into()
        //     .unwrap();

        // let _order_commitment: OrderCommitment = context
        //     .rpc_client()
        //     .request(
        //         context
        //             .config()
        //             .tx_orderer_rpc_url_list()
        //             .choose(&mut StdRng::seed_from_u64(seed))
        //             .ok_or(Error::EmptyTxOrdererRpcUrl)?,
        //         "send_raw_transaction",
        //         parameter,
        //         Id::Null,
        //     )
        //     .await
        //     .map_err(|e| {
        //         tracing::error!("Failed to send raw transaction: {:?}", e);
        //         e
        //     })?;

        // Ok(serde_json::to_value(raw_transaction_hash.as_string())?)
    }
}
