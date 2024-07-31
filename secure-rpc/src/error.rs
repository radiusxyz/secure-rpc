#[derive(Debug)]
pub enum Error {
    OpenConfig(std::io::Error),
    ParseConfig(toml::de::Error),
    JsonRPC(json_rpc::Error),
    Uninitialized,
    FetchResponse,
    InvalidSequencerPort,
    InvalidSecureRpcPort,

    LoadConfigOption,
    ParseTomlString,
    RemoveConfigDirectory,
    CreateConfigDirectory,
    CreateConfigFile,

    // Context
    KeyDoesNotExist,
    Downcast,
    NoneType,
}

unsafe impl Send for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<json_rpc::Error> for Error {
    fn from(value: json_rpc::Error) -> Self {
        Self::JsonRPC(value)
    }
}
