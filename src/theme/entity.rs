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

    pub fn article_index_template(&self) -> PathBuf {
        self.theme_path.join("article_index.html.hbs")
    }

    pub fn article_template(&self) -> PathBuf {
        self.theme_path.join("article.html.hbs")
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Theme;

    #[test]
    fn article_index_template_should_be_return_to_joined_path() {
        let theme = Theme::new("foo", "fixtures/foo");
        assert_eq!(
            theme.article_index_template(),
            PathBuf::new()
                .join("fixtures")
                .join("foo")
                .join("article_index.html.hbs")
        )
    }

    #[test]
    fn article_template_should_be_return_to_joined_path() {
        let theme = Theme::new("foo", "fixtures/foo");
        assert_eq!(
            theme.article_template(),
            PathBuf::new()
                .join("fixtures")
                .join("foo")
                .join("article.html.hbs")
        )
    }
}
