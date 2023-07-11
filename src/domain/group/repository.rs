use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::DomainError;

use super::model::{GroupCreateModel, GroupModel, GroupUpdateModel};

#[async_trait]
pub trait GroupRepository: Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<GroupModel>, u32)>, DomainError>;
    async fn find_by_groupid(&self, id: &Uuid) -> Result<Option<GroupModel>, DomainError>;
    async fn insert(
        &self,
        group_create_model: &GroupCreateModel,
    ) -> Result<GroupModel, DomainError>;
    async fn update_by_groupid(
        &self,
        id: &Uuid,
        group_update_model: &GroupUpdateModel,
    ) -> Result<GroupModel, DomainError>;
    async fn delete_by_groupid(&self, id: &Uuid) -> Result<(), DomainError>;
}
