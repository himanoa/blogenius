use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Theme {
    id: String,
    theme_path: PathBuf,
}

impl Theme {
    pub fn new(id: impl Into<String>, theme_path: impl Into<PathBuf>) -> Theme {
        Theme {
            id: id.into(),
            theme_path: theme_path.into(),
        }
    }
}
