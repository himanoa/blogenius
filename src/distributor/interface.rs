use std::path::PathBuf;

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriteFileError {
    #[error(transparent)]
    WriteFileError(#[from] std::io::Error),
}

pub trait Distributor {
    fn write(&self, dist_path: impl Into<PathBuf>, body: impl Into<String>) -> Result<()>;
}
