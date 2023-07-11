use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    article::{model::ArticleModel, repository::ArticleRepository},
    error::DomainError,
};


pub async fn execute(
    article_repository: Arc<dyn ArticleRepository>,
    id: Uuid,
) -> Result<Option<ArticleModel>, DomainError> {
    if let Some(article) = article_repository.find_by_articleid(&id).await? {
        return Ok(Some(article));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::domain::article::model::{ArticleCreateModel, ArticleUpdateModel, ArticleModel};

    use super::*;

    mock! {
        pub FakeArticleRepository { }

        #[async_trait]
        impl ArticleRepository for FakeArticleRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError>;
            async fn find_by_articleid(&self, id: &Uuid) -> Result<Option<ArticleModel>, DomainError>;
            async fn insert(&self,article_create_model: &ArticleCreateModel) -> Result<ArticleModel, DomainError>;
            async fn update_by_articleid(&self,id: &Uuid,article_update_model: &ArticleUpdateModel) -> Result<ArticleModel, DomainError>;
            async fn delete_by_articleid(&self, id: &Uuid) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_article_finded() {
        let mut article_repository = MockFakeArticleRepository::new();

        article_repository
            .expect_find_by_articleid()
            .return_once(|_| Ok(Some(ArticleModel::mock_default())));

        let result = execute(Arc::new(article_repository), Uuid::new_v4()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_aritcle() {
        let mut article_repository = MockFakeArticleRepository::new();

        article_repository
            .expect_find_by_articleid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(article_repository), Uuid::new_v4()).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}
