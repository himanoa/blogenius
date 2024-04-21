use std::{ops::Deref, path::PathBuf};

use crate::markdown::render;
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq)]
pub struct Article {
    pub id: ArticleId,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub draft: bool,
    pub author: String,

    pub rendered_body: String,
    pub old_id: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ArticleId(pub String);

impl ArticleId {
    pub fn new(id: impl Into<String>) -> ArticleId {
        ArticleId(id.into())
    }
}

impl Deref for ArticleId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct ArticleHeader {
    title: String,
    date: DateTime<FixedOffset>,
    draft: bool,
    author: String,
}

impl Article {
    pub fn new<Title, Author, RawBody>(
        id: ArticleId,
        title: Title,
        date: DateTime<FixedOffset>,
        draft: bool,
        author: Author,
        raw_body: RawBody,
        old_id: Option<String>,
    ) -> Article
    where
        Title: Into<String>,
        Author: Into<String>,
        RawBody: Into<String>
    {
        Article {
            id,
            title: title.into(),
            date,
            draft,
            author: author.into(),
            rendered_body: raw_body.into(),
            old_id: old_id.map(|i| i.into()),
        }
    }

    pub fn body(&self) -> Result<String> {
        return Ok(self.rendered_body.clone());
    }
}

impl TryFrom<(PathBuf, ArticleId, String)> for Article {
    type Error = anyhow::Error;

    fn try_from(value: (PathBuf, ArticleId, String)) -> Result<Self, Self::Error> {
        let (header, body) = render::<ArticleHeader>(&value.2)?;
        let old_id = value
            .0
            .file_stem()
            .and_then(|st| st.to_str().and_then(|s| s.parse::<u64>().ok()))
            .map(|s| s.to_string());
        let id = value.1;
        Ok(Article::new(
            id,
            header.title,
            header.date,
            header.draft,
            header.author,
            body,
            old_id,
        ))
    }
}
