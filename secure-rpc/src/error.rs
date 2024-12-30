use json_rpc::RpcServerError;
use radius_sdk::json_rpc::client::RpcClientError;

use crate::types::ConfigError;

#[derive(Debug)]
pub enum Error {
    Syscall(std::io::Error),

    Config(ConfigError),
    OpenConfig(std::io::Error),
    ParseConfig(toml::de::Error),
    JsonRPC(json_rpc::Error),
    Uninitialized,
    FetchResponse,
    InvalidSequencerPort,
    InvalidSecureRpcPort,

    DistributedKeyGenerationClient(RpcClientError),
    RpcServerError(RpcServerError),

    LoadConfigOption,
    ParseTomlString,
    RemoveConfigDirectory,
    CreateConfigDirectory,
    CreateConfigFile,

    // Context
    ContextUpdateFail,
    KeyDoesNotExist,
    Downcast,
    NoneType,

    FailedToGetSkdeParams,

    DecodeFailed,
    PvdeZkpInvalid,
    DecryptionError(skde::delay_encryption::DecryptionError),

    EncryptionNotEnabled,
    UnsupportedEncryptionType,
    UnsupportedDecryptionType,
    UnsupportedTransactionType,
}

unsafe impl Send for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<ConfigError> for Error {
    fn from(value: ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<json_rpc::Error> for Error {
    fn from(value: json_rpc::Error) -> Self {
        Self::JsonRPC(value)
    }
}

impl From<RpcServerError> for Error {
    fn from(value: RpcServerError) -> Self {
        Self::RpcServerError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Syscall(value)
    }
}
