use anyhow::Result;
use thiserror::Error;

use super::entity::Theme;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ThemeRepositoryResolveError {
    #[error("Theme({:?}) is not found", .0)]
    NotFoundError(String),
}

pub trait ThemeRepository {
    fn resolve(&self, id: &str) -> Result<Theme>;
    fn list(&self) -> Result<Theme>;
}
