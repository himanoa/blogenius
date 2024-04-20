use super::entity::{Article, ArticleId};

trait ArticleRepository {
    fn resolve(id: ArticleId) -> Option<Article>;
}
