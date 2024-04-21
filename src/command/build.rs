use crate::{
    article::repository::{ArticleRepository, HaveArticleRepository},
    config::Config,
    renderer::Renderer,
    theme::repository::HaveThemeRepository,
};
use anyhow::Result;
use std::fs::write;

pub fn build<T>(kernel: T, config: Config) -> Result<()>
where
    T: HaveArticleRepository + HaveThemeRepository,
{
    let articles = kernel.article_repository().list()?;

    for article in articles {
        println!("Start generate {} article", &*article.id);
        let rendered = kernel.render(&config.theme, &*article.id, &config.dist_path)?;
        write(rendered.dist_path, rendered.body)?;
        println!("End generate {} article", *article.id);
    }
    Ok(())
}
