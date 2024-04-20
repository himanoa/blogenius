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
        return render(&self.raw_body)
    }

    pub fn id(&self) -> String {
        return format!("{}-{}", self.title, self.date.to_utc().format("%Y/%m/%d-%H:%M")).to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::Article;

    #[test]
    pub fn test_id() {
        let article = Article::new(
            "foo",
            DateTime::parse_from_str("2022/01/30 21:00:00 +0900", "%Y/%m/%d %H:%M:%S %z").unwrap(),
            true,
            "himanoa",
            "empty",
            None
        );

        assert_eq!(article.id(), "foo-2022/01/30-12:00")
    }
}

