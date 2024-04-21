use std::path::PathBuf;
use std::fs::read_to_string;

use anyhow::Result;
use thiserror::Error;

use crate::article::repository::ArticleRepository;
use crate::article::entity::{ArticleId, Article};

pub struct FileArticleRepository {
    root_path: PathBuf 
}

impl FileArticleRepository {
    pub fn new(root_path: PathBuf) -> FileArticleRepository {
        FileArticleRepository { root_path }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FileArticleResolveError {
    #[error("article {:?} is not found", .0)]
    NotFound(String)
}
impl ArticleRepository for FileArticleRepository {
    fn resolve(&self, id: ArticleId) -> Result<Article> {
        let path = self.root_path.join(format!("{}.md", *id));
        let file_body = read_to_string(&path)?;
        let article = (path, file_body).try_into()?;
        Ok(article)
    }
}

#[cfg(test)]
pub mod tests {

    use std::path::Path;
    use chrono::DateTime;

    use crate::article::{entity::{Article, ArticleId}, repository::ArticleRepository};
    use super::FileArticleRepository;

    #[test]
    fn resolve_should_be_return_to_none_when_not_exist() {
        let repository = FileArticleRepository::new(Path::new(&"fixtures").to_path_buf());
        assert_eq!(repository.resolve(ArticleId::new("12")).ok(), None)
    }

    #[test]
    fn resolve_should_be_return_to_article() {
        let repository = FileArticleRepository::new(Path::new(&"fixtures").to_path_buf());
        assert_eq!(
            repository.resolve(
                ArticleId::new("example")
            ).expect("failed load example article"),
            Article::new(
                "example",
                DateTime::parse_from_str("2022/01/30 19:00:00 +0900", "%Y/%m/%d %H:%M:%S %z").expect("datetime"),
                false,
                "himanoa",
                "<p>example</p>\n",
                None
            )
        )
    }
    }
