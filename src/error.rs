use serde_yaml::Error as SerdeError;
use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load configuration file from {}", path.display())]
    LoadConfiguration {
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("invalid configuration file at {}", path.display())]
    InvalidConfiguration { source: SerdeError, path: PathBuf },
}
