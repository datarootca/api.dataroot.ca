use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{article::repository::ArticleRepository, error::DomainError};

pub async fn execute(
    article_repository: Arc<dyn ArticleRepository>,
    article_id: Uuid,
) -> Result<(), DomainError> {
    let has_article = article_repository.find_by_articleid(&article_id).await?;
    if has_article.is_none() {
        return Err(DomainError::NotFound(String::from("articleid not found")));
    }

    article_repository.delete_by_articleid(&article_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    use uuid::Uuid;

    use crate::domain::article::model::{
        ArticleCreateModel, ArticleModel, ArticleUpdateModel,
    };

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
    async fn it_should_return_void_article_deleted() {
        let mut article_repository = MockFakeArticleRepository::new();

        article_repository
            .expect_find_by_articleid()
            .return_once(|_| Ok(Some(ArticleModel::mock_default())));

        article_repository
            .expect_delete_by_articleid()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(article_repository), Uuid::new_v4()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_article_not_found() {
        let mut article_repository = MockFakeArticleRepository::new();

        article_repository
            .expect_find_by_articleid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(article_repository), Uuid::new_v4()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
