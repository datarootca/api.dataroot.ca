use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::DomainError;

use super::model::{ArticleCreateModel, ArticleModel, ArticleUpdateModel};

#[async_trait]
pub trait ArticleRepository: Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError>;
    async fn find_by_articleid(&self, id: &Uuid) -> Result<Option<ArticleModel>, DomainError>;
    async fn insert(
        &self,
        article_create_model: &ArticleCreateModel,
    ) -> Result<ArticleModel, DomainError>;
    async fn update_by_articleid(
        &self,
        id: &Uuid,
        article_update_model: &ArticleUpdateModel,
    ) -> Result<ArticleModel, DomainError>;
    async fn delete_by_articleid(&self, id: &Uuid) -> Result<(), DomainError>;
}
