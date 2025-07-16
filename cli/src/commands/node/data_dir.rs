use std::path::PathBuf;

use crate::Args;

#[derive(Debug, Args)]
pub struct DataDirArgs {
    #[arg(long = "datadir.db")]
    pub db_path: Option<PathBuf>,
    #[arg(long = "datadir.private-key")]
    pub private_key: Option<PathBuf>,
    #[arg(long = "datadir.trusted-setup")]
    pub trusted_setup: Option<PathBuf>,
}
