use std::{fs::read_to_string, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use toml::from_str;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub dist_path: String,
    pub article_path: String,
    pub theme_path: String,
    pub theme: String,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ConfigLoadError {
    #[error("Config not found")]
    NotFound,
    #[error("Failed parse config toml")]
    ParseFailed,
}

impl Config {
    pub fn load(path: impl Into<PathBuf>) -> Result<Config> {
        let path: PathBuf = path.into();
        let config = read_to_string(&path).map_err(|_| ConfigLoadError::NotFound)?;
        Ok(from_str(&config).map_err(|_| ConfigLoadError::ParseFailed)?)
    }

    pub fn new(
        dist_path: impl Into<String>,
        article_path: impl Into<String>,
        theme_path: impl Into<String>,
        theme: impl Into<String>,
    ) -> Config {
        Config {
            dist_path: dist_path.into(),
            article_path: article_path.into(),
            theme_path: theme_path.into(),
            theme: theme.into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new("dist", "articles", "theme", "default")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn load_should_be_return_to_ok_when_file_exist() {
        let config = Config::load("./fixtures/config.toml").expect("config load error");
        assert_eq!(config.dist_path, "dist");
        assert_eq!(config.article_path, "articles");
        assert_eq!(config.theme_path, "theme");
        assert_eq!(config.theme, "foo");
    }

    #[test]
    fn load_should_be_return_to_not_found_when_file_not_exist() {
        let config = Config::load("./fixtures/config_not_found.toml").unwrap_err();
        assert_eq!(config.to_string(), "Config not found")
    }

    #[test]
    fn load_should_be_return_to_not_found_when_invalid_json() {
        let config = Config::load("./fixtures/invalid_config.toml").unwrap_err();
        assert_eq!(config.to_string(), "Failed parse config toml");
    }
}
