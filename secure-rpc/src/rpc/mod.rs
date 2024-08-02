pub mod external;
pub mod prelude {
    pub use std::sync::Arc;

    pub use json_rpc::{RpcClient, RpcError, RpcParameter};
    pub use sequencer::types::*;
    pub use serde::{Deserialize, Serialize};

    pub use crate::error::Error;
}
