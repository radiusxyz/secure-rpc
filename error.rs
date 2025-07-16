use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("System IO error: {0}")]
    Syscall(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(#[from] crate::types::config::ConfigError),

    #[error("RPC server error: {0}")]
    RpcServer(#[from] radius_sdk::json_rpc::server::RpcServerError),

    #[error("Uninitialized state")]
    Uninitialized,

    #[error("Failed to fetch response")]
    FetchResponse,

    #[error("Invalid tx-orderer port")]
    InvalidTxOrdererPort,

    #[error("Invalid secure RPC port")]
    InvalidSecureRpcPort,

    #[error("Raw transaction list is empty")]
    EmptyRawTransaction,

    #[error("TxOrderer RPC URL is empty")]
    EmptyTxOrdererRpcUrl,

    #[error("Distributed Key Generation error: {0}")]
    DistributedKeyGenerationClient(#[from] radius_sdk::json_rpc::client::RpcClientError),

    #[error("Failed to load config option")]
    LoadConfigOption,

    #[error("Failed to parse TOML config string")]
    ParseTomlString,

    #[error("Failed to remove config directory")]
    RemoveConfigDirectory,

    #[error("Failed to create config directory")]
    CreateConfigDirectory,

    #[error("Failed to create config file")]
    CreateConfigFile,

    // Context-related
    #[error("Failed to update context")]
    ContextUpdateFail,

    #[error("Requested key does not exist")]
    KeyDoesNotExist,

    #[error("Failed to downcast dynamic type")]
    Downcast,

    #[error("Unexpected None value")]
    NoneType,

    #[error("Failed to get SKDE parameters")]
    FailedToGetSkdeParams,

    #[error("Decryption failed: {0}")]
    DecryptionError(#[from] skde::delay_encryption::DecryptionError),

    #[error("Failed to encrypt transaction: {0}")]
    EncryptionError(#[from] skde::delay_encryption::EncryptionError),

    #[error("Failed to decode encrypted payload")]
    DecodeFailed,

    #[error("Encryption not enabled")]
    EncryptionNotEnabled,

    #[error("Unsupported encryption type")]
    UnsupportedEncryptionType,

    #[error("Unsupported decryption type")]
    UnsupportedDecryptionType,

    #[error("Unsupported transaction type")]
    UnsupportedTransactionType,

    #[error("Failed to intialize distributed key generation client")]
    DistributedKeyGenerationClientNotInitialized,

    #[error("Serialization error")]
    SerializationError,
}
