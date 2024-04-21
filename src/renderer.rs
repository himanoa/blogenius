use std::{iter::Map, path::PathBuf};

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use anyhow::Result;

use crate::{article::{entity::ArticleId, file_article_repository::FileArticleResolveError, repository::{ArticleRepository, HaveArticleRepository}}, theme::repository::{HaveThemeRepository, ThemeRepository, ThemeRepositoryResolveError}};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RenderError {
    #[error(transparent)]
    ArticleRepositoryError(#[from] FileArticleResolveError),
    #[error(transparent)]
    ThemeRepositoryError(#[from] ThemeRepositoryResolveError)
}

pub struct RenderResult {
    pub body: String,
    pub dist_path: PathBuf
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvideValue { 
    body: String
}

impl ProvideValue {
    fn new(body: impl Into<String>) -> ProvideValue {
        ProvideValue { body: body.into() }
    }
}

pub trait Renderer: HaveThemeRepository + HaveArticleRepository {
    fn render(&self, theme_name: impl Into<String>, article_id: impl Into<String>, dist_path: PathBuf) -> Result<RenderResult>  {
        let mut handlebars = Handlebars::new();
        let article = self.article_repository().resolve(ArticleId::new(article_id))?;
        let theme = self.theme_repository().resolve(&theme_name.into())?;
        let article_template_path = theme.article_template();
        let provide_value = ProvideValue::new(article.rendered_body);

        handlebars.register_template_file("template", article_template_path)?;

        let rendered = handlebars.render_template("template", &provide_value)?;
        Ok(RenderResult { body: rendered, dist_path: todo!() })
    }
}
