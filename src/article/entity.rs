use chrono::{DateTime, FixedOffset};
use crate::markdown::render;

#[derive(Debug, Eq, PartialEq)]
pub struct Article {
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub draft: bool,
    pub author: String,

    pub raw_body: String,
    pub old_id: Option<String>
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArticleId {
    OldId(String),
    NewId(String)
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
            raw_body: raw_body.into(),
            old_id: old_id.map(|i| i.into())
        }
    }
    pub fn body(&self) -> String {
        todo!()
        // return render(&self.raw_body)
    }

    pub fn id(&self) -> ArticleId {
        self.old_id.clone()
            .map(|id| ArticleId::OldId(id))
            .unwrap_or(
                ArticleId::NewId(format!("{}-{}",
                    &self.title,
                    &self.date.to_utc().format("%Y/%m/%d-%H:%M")).to_string())
                )
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::article::entity::ArticleId;

    use super::Article;

    #[test]
    pub fn test_id_should_be_return_to_new_id() {
        let article = Article::new(
            "foo",
            DateTime::parse_from_str("2022/01/30 21:00:00 +0900", "%Y/%m/%d %H:%M:%S %z").unwrap(),
            true,
            "himanoa",
            "empty",
            None
        );

        assert_eq!(article.id(), ArticleId::NewId("foo-2022/01/30-12:00".to_string()))
    }

    #[test]
    pub fn test_id_should_be_return_to_old_id() {
        let article = Article::new(
            "foo",
            DateTime::parse_from_str("2022/01/30 21:00:00 +0900", "%Y/%m/%d %H:%M:%S %z").unwrap(),
            true,
            "himanoa",
            "empty",
            Some("1".to_string())
        );

        assert_eq!(article.id(), ArticleId::OldId("1".to_string()))
    }
}

