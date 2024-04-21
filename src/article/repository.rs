use super::entity::{Article, ArticleId};
use anyhow::Result;

pub trait ArticleRepository {
    fn resolve(&self, id: ArticleId) -> Result<Article>;
    fn list(&self) -> Result<Vec<Article>>;
}

pub trait HaveArticleRepository {
    type ArticleRepository: ArticleRepository + Send + Sync + 'static;

    fn article_repository(&self) -> &Self::ArticleRepository;
}
