use std::{ops::Deref, path::PathBuf};

use chrono::{DateTime, FixedOffset};
use anyhow::Result;
use serde::Deserialize;
use crate::markdown::render;

#[derive(Debug, Eq, PartialEq)]
pub struct Article {
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub draft: bool,
    pub author: String,

    pub rendered_body: String,
    pub old_id: Option<String>
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
    pub fn new(
        title: impl Into<String>,
        date: DateTime<FixedOffset>,
        draft: bool,
        author: impl Into<String>,
        raw_body: impl Into<String>,
        old_id: Option<String>
    ) -> Article  {
        Article { 
            title: title.into(),
            date,
            draft,
            author: author.into(),
            rendered_body: raw_body.into(),
            old_id: old_id.map(|i| i.into())
        }
    }

    pub fn body(&self) -> Result<String> {
        return Ok(self.rendered_body.clone())
    }
}

impl TryFrom<(PathBuf, String)> for Article {
    type Error = anyhow::Error;

    fn try_from(value: (PathBuf, String)) -> Result<Self, Self::Error> {
        let (header, body) = render::<ArticleHeader>(&value.1)?;
        let old_id = value.0.file_stem().and_then(|st| {
            st.to_str().and_then(|s| s.parse::<u64>().ok())
        }).map(|s| s.to_string());
        Ok(Article::new(header.title, header.date, header.draft, header.author, body, old_id))
    }
}

#[cfg(test)]
mod tests {
    pub fn test() {
        assert_eq!(1 + 1, 2)
    }
}

