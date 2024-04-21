use anyhow::Result;
use thiserror::Error;

use super::entity::Theme;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ThemeRepositoryResolveError {
    #[error("Theme({:?}) is not found", .0)]
    NotFoundError(String),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ThemeRepositoryListError {
    #[error("Theme directory is not found")]
    NotFoundThemeDirectory,
}

pub trait ThemeRepository {
    fn resolve(&self, id: &str) -> Result<Theme>;
    fn list(&self) -> Result<Vec<String>>;
}

pub trait HaveThemeRepository {
    type ThemeRepository: ThemeRepository + Send + Sync + 'static;

    fn theme_repository(&self) -> &Self::ThemeRepository;
}
