use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Theme {
    id: String,
    articles_path: PathBuf,
    article_path: PathBuf,
}

impl Theme {
    fn new(id: impl Into<String>, articles_path: PathBuf, article_path: PathBuf) -> Theme {
        Theme { id: id.into(), article_path, articles_path }
    }
}
