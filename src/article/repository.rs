use super::entity::{Article, ArticleId};

pub trait ArticleRepository {
    fn resolve(&self, id: ArticleId) -> Option<Article>;
}
