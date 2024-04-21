use anyhow::{bail, Result};
use std::fs::read_dir;
use std::path::PathBuf;

use super::{
    entity::Theme,
    repository::{ThemeRepository, ThemeRepositoryListError, ThemeRepositoryResolveError},
};

pub struct FileThemeRepository {
    root_path: PathBuf,
}

impl FileThemeRepository {
    pub fn new(root_path: impl Into<PathBuf>) -> FileThemeRepository {
        FileThemeRepository {
            root_path: root_path.into(),
        }
    }
}

impl ThemeRepository for FileThemeRepository {
    fn resolve(&self, id: &str) -> Result<super::entity::Theme> {
        let theme_dir = self.root_path.join(&id);
        if theme_dir.is_dir() {
            Ok(Theme::new(id, theme_dir))
        } else {
            bail!(ThemeRepositoryResolveError::NotFoundError(
                "xxx".to_string()
            ))
        }
    }

    fn list(&self) -> Result<Vec<String>> {
        let entries = read_dir(&self.root_path)
            .map_err(|_| ThemeRepositoryListError::NotFoundThemeDirectory)?;
        let themes = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type().ok()?;
                if file_type.is_dir() {
                    entry.file_name().into_string().ok()
                } else {
                    None
                }
            })
            .collect();
        Ok(themes)
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::{entity::Theme, repository::ThemeRepository};

    use super::FileThemeRepository;

    #[test]
    fn resolve_should_be_return_to_ok_and_theme_when_exist_theme_dir() {
        let repository = FileThemeRepository::new("fixtures/themes");
        assert_eq!(
            repository.resolve("dummy").unwrap(),
            Theme::new("dummy", "fixtures/themes/dummy")
        )
    }

    #[test]
    fn resolve_should_be_return_to_not_found_error_when_not_exist_theme_dir() {
        let repository = FileThemeRepository::new("fixtures/themes");
        assert_eq!(
            repository
                .resolve("dummy_not_found")
                .unwrap_err()
                .to_string(),
            "Theme(\"xxx\") is not found"
        );
    }

    #[test]
    fn list_should_be_return_to_ok_themes() {
        let repository = FileThemeRepository::new("./fixtures/themes");
        assert_eq!(repository.list().unwrap(), vec!["dummy"]);
    }

    #[test]
    fn list_should_be_return_to_err() {
        let repository = FileThemeRepository::new("./fixtures/themes_not_found");

        assert_eq!(
            repository.list().unwrap_err().to_string(),
            "Theme directory is not found"
        );
    }
}
