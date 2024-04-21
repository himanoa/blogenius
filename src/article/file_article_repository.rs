use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use glob::{glob, Paths};
use thiserror::Error;

use crate::article::entity::{Article, ArticleId};
use crate::article::repository::ArticleRepository;

pub struct FileArticleRepository {
    root_path: PathBuf,
}

impl FileArticleRepository {
    pub fn new(root_path: PathBuf) -> FileArticleRepository {
        FileArticleRepository { root_path }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FileArticleResolveError {
    #[error("article {:?} is not found", .0)]
    NotFound(String),
}

impl ArticleRepository for FileArticleRepository {
    fn resolve(&self, id: ArticleId) -> Result<Article> {
        let path = self.root_path.join(format!("{}.md", *id));
        let file_body = read_to_string(&path)?;
        let article = (path, id, file_body).try_into()?;
        Ok(article)
    }

    fn list(&self) -> Result<Vec<Article>> {
        let paths = glob(&self.root_path.join("*.md").to_string_lossy())?
            .map(|entry| entry.context("Failed to get file path"))
            .collect::<Result<Vec<_>>>()?;
        paths
            .into_iter()
            .map(|p| {
                let article_id = p.file_stem().expect("filestem is not found");
                self.resolve(ArticleId::new(article_id.to_string_lossy()))
            })
            .collect::<Result<Vec<Article>>>()
    }
}

#[cfg(test)]
pub mod tests {

    use chrono::DateTime;
    use std::path::Path;

    use super::FileArticleRepository;
    use crate::article::{
        entity::{Article, ArticleId},
        repository::ArticleRepository,
    };

    #[test]
    fn resolve_should_be_return_to_none_when_not_exist() {
        let repository = FileArticleRepository::new(Path::new(&"fixtures").to_path_buf());
        assert_eq!(repository.resolve(ArticleId::new("12")).ok(), None)
    }

    #[test]
    fn resolve_should_be_return_to_article() {
        let repository = FileArticleRepository::new(Path::new(&"fixtures").to_path_buf());
        assert_eq!(
            repository
                .resolve(ArticleId::new("example"))
                .expect("failed load example article"),
            Article::new(
                ArticleId::new("example"),
                "example",
                DateTime::parse_from_str("2022/01/30 19:00:00 +0900", "%Y/%m/%d %H:%M:%S %z")
                    .expect("datetime"),
                false,
                "himanoa",
                "<p>example</p>\n",
                None
            )
        )
    }

    #[test]
    fn list_should_be_return_to_articles() {
        let repository = FileArticleRepository::new(Path::new(&"fixtures").to_path_buf());
        let expected = vec![Article::new(
            ArticleId::new("example"),
            "example",
            DateTime::parse_from_str("2022/01/30 19:00:00 +0900", "%Y/%m/%d %H:%M:%S %z")
                .expect("datetime"),
            false,
            "himanoa",
            "<p>example</p>\n",
            None,
        )];
        assert_eq!(
            repository.list().expect("failed load example article"),
            expected
        )
    }
}
