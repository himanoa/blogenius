use std::path::PathBuf;

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

// TODO; implemented
impl ArticleRepository for FileArticleRepository {
    fn resolve(&self, id: ArticleId) -> Option<Article> {
        match id {
            ArticleId::OldId(id) => {
                let _path = self.root_path.join(format!("{}.md", id));
                todo!()
            }
            _ => todo!()
        }
    }
}

#[cfg(test)]
pub mod tests {

    use std::path::Path;

    use crate::article::{entity::ArticleId, repository::ArticleRepository};

    use super::FileArticleRepository;

    #[test]
    fn resolve_should_be_return_to_none_when_not_exist() {
        let repository = FileArticleRepository::new(Path::new(&"articles").to_path_buf());
        assert_eq!(repository.resolve(ArticleId::OldId("12".to_string())), None)
    }
}
