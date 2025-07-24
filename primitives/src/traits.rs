/// Trait for the RPC for smoothly plugged into `RpcParameter`
pub trait RpcT: Clone + Send + Sync + 'static {}
impl <T: Send + Sync + Clone + 'static> RpcT for T {}

/// Error trait for the RPC service for thread safe error handling
pub trait ErrorT: std::error::Error + Send + 'static {}
impl <T: std::error::Error + Send + 'static> ErrorT for T {}