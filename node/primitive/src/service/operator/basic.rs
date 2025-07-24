use secure_rpc_primitives::{Context, OperatorService, TrustedSetupFor};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct BasicOperatorService<C> {
    dkg_rpc_urls: Vec<String>,
    _context: C,
}

impl<C: Context> BasicOperatorService<C> {
    pub fn new(dkg_rpc_urls: Vec<String>, context: C) -> Self {
        Self { dkg_rpc_urls, _context: context }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BasicOperatorServiceError {
    #[error("Failed to get trusted setup")]
    FailedToGetTrustedSetup,
}

#[async_trait]
impl<C: Context> OperatorService for BasicOperatorService<C> {
    type TrustedSetup = TrustedSetupFor<C>;
    type Task = ();
    type Error = BasicOperatorServiceError;

    async fn get_trusted_setup(&self) -> Option<Self::TrustedSetup> {
        None
    }

    async fn get_operator_rpc_urls(&self) -> Option<Vec<String>> {
        Some(self.dkg_rpc_urls.clone())
    }

    async fn create_task(&self) -> Result<Self::Task, Self::Error> {
        todo!()
    }

    async fn respond_task(&self, _task: ()) -> Result<(), Self::Error> {
        todo!()
    }
}