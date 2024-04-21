use std::path::PathBuf;

use anyhow::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    article::{
        entity::ArticleId,
        file_article_repository::FileArticleResolveError,
        repository::{ArticleRepository, HaveArticleRepository},
    },
    theme::repository::{HaveThemeRepository, ThemeRepository, ThemeRepositoryResolveError},
};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RenderError {
    #[error(transparent)]
    ArticleRepositoryError(#[from] FileArticleResolveError),
    #[error(transparent)]
    ThemeRepositoryError(#[from] ThemeRepositoryResolveError),
}

pub struct RenderResult {
    pub body: String,
    pub dist_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvideValue {
    body: String,
}

impl ProvideValue {
    fn new(body: impl Into<String>) -> ProvideValue {
        ProvideValue { body: body.into() }
    }
}

pub trait Renderer: HaveThemeRepository + HaveArticleRepository {
    fn render(
        &self,
        theme_name: impl Into<String>,
        article_id: impl Into<String>,
        dist_path: impl Into<PathBuf>,
    ) -> Result<RenderResult> {
        let mut handlebars = Handlebars::new();
        let article = self
            .article_repository()
            .resolve(ArticleId::new(article_id))?;
        let article_template = self
            .theme_repository()
            .get_article_template(&theme_name.into())?;
        let provide_value = ProvideValue::new(article.rendered_body);

        handlebars.register_escape_fn(handlebars::no_escape);

        let rendered = handlebars.render_template(&article_template, &provide_value)?;
        Ok(RenderResult {
            body: rendered,
            dist_path: dist_path
                .into()
                .join(format!("{}.html", article.id.to_string())),
        })
    }
}

impl<U: HaveThemeRepository + HaveArticleRepository> Renderer for U {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::Utc;

    use crate::{
        article::{
            entity::Article,
            repository::{ArticleRepository, HaveArticleRepository},
        },
        theme::{
            entity::Theme,
            repository::{HaveThemeRepository, ThemeRepository},
        },
    };

    use super::Renderer;

    struct DummyThemeRepository {}

    impl ThemeRepository for DummyThemeRepository {
        fn resolve(&self, id: &str) -> anyhow::Result<crate::theme::entity::Theme> {
            Ok(Theme::new(id, PathBuf::new()))
        }

        fn list(&self) -> anyhow::Result<Vec<String>> {
            unreachable!()
        }

        fn get_article_template(&self, _id: &str) -> anyhow::Result<String> {
            Ok("{{body}}".to_string())
        }
    }

    struct DummyArticleRepository {}

    impl ArticleRepository for DummyArticleRepository {
        fn resolve(
            &self,
            id: crate::article::entity::ArticleId,
        ) -> anyhow::Result<crate::article::entity::Article> {
            Ok(Article::new(
                id,
                "example",
                Utc::now().into(),
                false,
                "himanoa",
                "<p>hello world</p>",
                None,
            ))
        }

        fn list(&self) -> anyhow::Result<Vec<Article>> {
            unreachable!()
        }
    }

    struct TestKernal {}

    impl HaveArticleRepository for TestKernal {
        type ArticleRepository = DummyArticleRepository;

        fn article_repository(&self) -> &Self::ArticleRepository {
            &DummyArticleRepository {}
        }
    }

    impl HaveThemeRepository for TestKernal {
        type ThemeRepository = DummyThemeRepository;

        fn theme_repository(&self) -> &Self::ThemeRepository {
            &DummyThemeRepository {}
        }
    }

    #[test]
    fn render_should_be_return_to_rendered_text() {
        let kernel = TestKernal {};
        let rendered = kernel.render("foo", "dummy", "dist").expect("render_error");
        assert_eq!(rendered.body, "<p>hello world</p>");
    }
}
