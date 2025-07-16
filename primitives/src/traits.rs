
pub trait RpcT: Clone + Send + Sync + 'static {}
impl <T: Send + Sync + Clone + 'static> RpcT for T {}