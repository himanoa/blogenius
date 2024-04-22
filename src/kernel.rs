use crate::{article::{file_article_repository::FileArticleRepository, repository::HaveArticleRepository}, config::Config, distributor::{file_distributor::FileDistributor, interface::HaveDistributor}, theme::{file_theme_repository::FileThemeRepository, repository::HaveThemeRepository}};

pub struct Kernel {
    file_article_repository: FileArticleRepository,
    file_theme_repository: FileThemeRepository,
    file_distributor: FileDistributor,
    pub config: Config
}

impl Kernel {
    pub fn new(config: Config) -> Kernel {
        Kernel {
            file_article_repository: FileArticleRepository::new(config.article_path.clone().into()),
            file_theme_repository: FileThemeRepository::new(&config.theme_path),
            file_distributor: FileDistributor::new(),
            config
        }
    }
}

impl HaveArticleRepository for Kernel {
    type ArticleRepository = FileArticleRepository;

    fn article_repository(&self) -> &Self::ArticleRepository {
        &self.file_article_repository
    }
}

impl HaveThemeRepository for Kernel {
    type ThemeRepository = FileThemeRepository;

    fn theme_repository(&self) -> &Self::ThemeRepository {
        &self.file_theme_repository
    }
}

impl HaveDistributor for Kernel {
    type Distributor = FileDistributor;

    fn distributor(&self) -> &Self::Distributor {
        &self.file_distributor
    }
}
