use secure_rpc_primitives::Context;

pub struct SecureRpcWorker;

pub async fn run_secure_rpc_worker<C: Context>(ctx: &C, worker: SecureRpcWorker) -> anyhow::Result<()> {
    Ok(())
} 

impl SecureRpcWorker {   
    pub fn run() {}
}