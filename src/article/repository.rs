use super::entity::{Article, ArticleId};
use anyhow::Result;

pub trait ArticleRepository {
    fn resolve(&self, id: ArticleId) -> Result<Article>;
}
