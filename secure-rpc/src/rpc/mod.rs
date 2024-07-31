pub mod external;
pub mod prelude {
    pub use std::sync::Arc;

    pub use json_rpc::{types::*, RpcClient, RpcError};
    pub use sequencer::types::*;
    pub use serde::{Deserialize, Serialize};

    pub use crate::error::Error;
}
