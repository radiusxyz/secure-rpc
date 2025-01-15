#[derive(Debug)]
pub enum Error {
    Syscall(std::io::Error),
    Config(crate::types::config::ConfigError),
    RpcServer(radius_sdk::json_rpc::server::RpcServerError),
    Uninitialized,
    FetchResponse,
    InvalidSequencerPort,
    InvalidSecureRpcPort,

    EmptyRawTransaction,

    DistributedKeyGenerationClient(radius_sdk::json_rpc::client::RpcClientError),
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

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Syscall(value)
    }
}

impl From<crate::types::config::ConfigError> for Error {
    fn from(value: crate::types::config::ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<radius_sdk::json_rpc::server::RpcServerError> for Error {
    fn from(value: radius_sdk::json_rpc::server::RpcServerError) -> Self {
        Self::RpcServer(value)
    }
}
