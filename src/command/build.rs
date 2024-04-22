use crate::{
    article::repository::{ArticleRepository, HaveArticleRepository},
    config::Config,
    distributor::interface::{Distributor, HaveDistributor},
    renderer::Renderer,
    theme::repository::HaveThemeRepository,
};
use anyhow::Result;

pub fn build<T>(kernel: &T, config: Config) -> Result<()>
where
    T: HaveArticleRepository + HaveThemeRepository + HaveDistributor,
{
    let articles = kernel.article_repository().list()?;
    let distributor = kernel.distributor();

    for article in articles {
        println!("Start generate {} article", &*article.id);
        let rendered = kernel.render(&config.theme, &*article.id, &config.dist_path)?;
        distributor.write(rendered.dist_path, rendered.body)?;
        println!("End generate {} article", *article.id);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, path::PathBuf, sync::Mutex};

    use chrono::Utc;

    use crate::{
        article::{
            entity::{Article, ArticleId},
            repository::{ArticleRepository, HaveArticleRepository},
        },
        config::Config,
        distributor::interface::{Distributor, HaveDistributor},
        theme::{
            entity::Theme,
            repository::{HaveThemeRepository, ThemeRepository},
        },
    };

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
            Ok(vec![Article::new(
                ArticleId::new("test"),
                "example",
                Utc::now().into(),
                false,
                "himanoa",
                "<p>hello world</p>",
                None,
            )])
        }
    }

    struct DummyDistributor {
        pub last_call: Mutex<Cell<Option<(PathBuf, String)>>>,
    }

    impl Distributor for DummyDistributor {
        fn write(
            &self,
            dist_path: impl Into<PathBuf>,
            body: impl Into<String>,
        ) -> anyhow::Result<()> {
            let cell = &self.last_call.lock().unwrap();
            cell.set(Some((dist_path.into(), body.into())));
            Ok(())
        }
    }

    struct TestKernal {
        dummy_distributor: DummyDistributor,
    }

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

    impl HaveDistributor for TestKernal {
        type Distributor = DummyDistributor;

        fn distributor(&self) -> &Self::Distributor {
            &self.dummy_distributor
        }
    }

    #[test]
    fn build_should_be_call_distributor_write() {
        let kernel = TestKernal {
            dummy_distributor: DummyDistributor {
                last_call: Mutex::new(Cell::new(None)),
            },
        };
        let _ = build(&kernel, Config::default());
        assert_eq!(
            kernel
                .dummy_distributor
                .last_call
                .lock()
                .unwrap()
                .take()
                .unwrap(),
            (
                PathBuf::new().join("dist/test.html"),
                "<p>hello world</p>".to_string()
            )
        );
    }
}
