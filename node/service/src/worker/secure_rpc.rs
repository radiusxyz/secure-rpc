use secure_rpc_primitives::{Context, ExternalRpcInterface, Operator, SecureRpcService, TrustedSetupFor};
use secure_rpc_node_primitive::RpcHandlerEvent;
use tokio::{sync::mpsc, task::JoinHandle};
use rand::{rngs::StdRng, SeedableRng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn start_secure_rpc_worker<C: Context, O: Operator>(context: &C, operator: O, tx_orderer_rpc_url: String) -> (JoinHandle<()>, mpsc::Sender<RpcHandlerEvent>) 
where
    O: Send + Sync + 'static,
    O::TrustedSetup: Into<TrustedSetupFor<C>>,
{
    let (mut worker, tx) = SecureRpcWorker::<C, O>::new(context.clone(), operator, tx_orderer_rpc_url);
    let handle = tokio::spawn(async move {
        worker.run().await;
    });
    (handle, tx)
}

/// Main Secure RPC worker that handles the events from other workers
struct SecureRpcWorker<C: Context, O: Operator> {
    context: C,
    operator: O,
    operator_urls: Option<Vec<String>>,
    tx_orderer_rpc_url: String,
    from_rpc_event_rx: mpsc::Receiver<RpcHandlerEvent>,
}

impl<C: Context, O: Operator> SecureRpcWorker<C, O> 
where
    O::TrustedSetup: Into<TrustedSetupFor<C>>,
{
    pub fn new(context: C, operator: O, tx_orderer_rpc_url: String) -> (Self, mpsc::Sender<RpcHandlerEvent>) {
        let (tx, from_rpc_event_rx) = mpsc::channel(100);
        (Self { context, operator, operator_urls: None, tx_orderer_rpc_url, from_rpc_event_rx }, tx)
    }

    async fn update_operator_list(&mut self) {
        if let Some(operator_urls) = self.operator.get_operator_rpc_urls().await {
            self.operator_urls = Some(operator_urls);
        }
    }

    async fn update_trusted_setup(&mut self) {
        if let Some(trusted_setup) = self.operator.get_active_trusted_setup().await {
            self.context.secure_rpc_service_mut().update_trusted_setup(trusted_setup.into());
        }
    }

    fn random_index(&self, size: usize) -> anyhow::Result<usize> {
        if size == 0 { return Err(anyhow::anyhow!("Empty RPC list")) }
        if size == 1 { return Ok(0) }
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        Ok(rng.gen_range(0..size))
    }

    fn get_random_operator_url(&self) -> anyhow::Result<String> {
        if let Some(operator_urls) = &self.operator_urls {
            let index = self.random_index(operator_urls.len())?;
            Ok(operator_urls[index].clone())
        } else {
            // TODO: Should handle this case. Should not reach here
            Err(anyhow::anyhow!("Not initialized?"))
        }
    }

    /// Encrypt the raw transaction if the encryption mode is enabled
    /// 1. Choose a random DKG RPC URL to get the encryption key
    /// 2. Encrypt the raw transaction with the encryption key
    /// 3. Serialize the encrypted transaction
    /// 4. Return the encrypted transaction
    pub async fn encrypt_tx(&self, raw_tx: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let operator_url = self.get_random_operator_url()?;
        let (enc_key, session_id) = self.context.external_rpc_service().get_enc_key(&operator_url).await.map_err(|e| anyhow::anyhow!("Failed to get encryption key: {}", e))?;
        let res = self.context.secure_rpc_service().encrypt_tx(session_id, &raw_tx[..], &enc_key)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to encrypt transaction: {}", e))?;
        Ok(serde_json::to_vec(&res).map_err(|e| anyhow::anyhow!("Failed to serialize encrypted transaction: {}", e))?)
    }

    /// Handle the sent transaction based on the encryption mode
    pub async fn handle_send_tx(&self, raw_tx: Vec<u8>, should_encrypt: bool) -> anyhow::Result<serde_json::Value> {
        let tx_data = if should_encrypt {
            self.encrypt_tx(raw_tx).await.map_err(|e| anyhow::anyhow!("Failed to encrypt transaction: {}", e))?
        } else {
            raw_tx
        };
        self.context.external_rpc_service().forward_tx(&self.tx_orderer_rpc_url, &self.context.rollup_id(), should_encrypt, tx_data).await.map_err(|e| anyhow::anyhow!("Failed to forward transaction: {}", e))
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.from_rpc_event_rx.recv().await {
            match event {
                RpcHandlerEvent::SendTx { raw_tx, should_encrypt, sender } => {
                    if let Ok(res) = self.handle_send_tx(raw_tx, should_encrypt).await {
                        if let Err(e) = sender.send(res) { tracing::error!("Channel is closed: {}", e); }
                    } else {
                        tracing::error!("Failed to forward transaction");
                    }
                }
            }
        }
    }
}