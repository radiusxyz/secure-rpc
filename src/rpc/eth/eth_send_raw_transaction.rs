use std::time::{SystemTime, UNIX_EPOCH};

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde_json::Value;
use tx_orderer::types::EthRawTransaction;

use crate::{
    rpc::{
        prelude::*, send_encrypted_transaction::SendEncryptedTransactionRequest,
        send_raw_transaction::SendRawTransaction, EncryptTransaction,
    },
    types::transaction::EncryptedTransactionType,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction(pub Vec<String>);

impl RpcParameter<AppState> for EthSendRawTransaction {
    type Response = Value;

    fn method() -> &'static str {
        "eth_sendRawTransaction"
    }

    async fn handler(self, context: AppState) -> Result<Value, RpcError> {
        let raw_transaction_str = self.0.get(0).ok_or(Error::EmptyRawTransaction)?.to_owned();

        let eth_raw_transaction = EthRawTransaction(raw_transaction_str.clone());
        let transaction_hash = eth_raw_transaction.raw_transaction_hash();
        let rollup_id = context.config().rollup_id().to_owned();

        let rpc_url = {
            let seed: u64 = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .try_into()
                .unwrap();

            context
                .config()
                .tx_orderer_rpc_url_list()
                .choose(&mut StdRng::seed_from_u64(seed))
                .ok_or(Error::EmptyTxOrdererRpcUrl)?
                .to_owned()
        };

        let transaction_hash_value = serde_json::to_value(transaction_hash.as_string())?;

        match context.config().encrypted_transaction_type() {
            EncryptedTransactionType::Skde => {
                let param = SendRawTransaction {
                    rollup_id,
                    raw_transaction: tx_orderer::types::RawTransaction::Eth(eth_raw_transaction),
                };

                context
                    .rpc_client()
                    .request::<_, Value>(rpc_url, "send_raw_transaction", param, Id::Null)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to send raw transaction: {:?}", e);
                        e
                    })?;

                Ok(transaction_hash_value)
            }
            _ => {
                // Handle encrypted transaction
                let encrypt_req = EncryptTransaction {
                    raw_transaction: tx_orderer::types::RawTransaction::Eth(eth_raw_transaction),
                };

                let encrypt_res = encrypt_req.handler(context.clone()).await?;

                let param = SendEncryptedTransactionRequest {
                    rollup_id,
                    encrypted_transaction: encrypt_res.encrypted_transaction,
                };

                context
                    .rpc_client()
                    .request::<_, Value>(rpc_url, "send_encrypted_transaction", param, Id::Null)
                    .await
                    .map(|_| transaction_hash_value)
                    .map_err(|e| {
                        tracing::error!("Failed to send encrypted transaction: {:?}", e);
                        e.into()
                    })
            }
        }
    }
}
