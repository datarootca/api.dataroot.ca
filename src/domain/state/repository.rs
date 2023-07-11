use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::DomainError;

use super::model::{StateCreateModel, StateModel, StateUpdateModel};

#[async_trait]
pub trait StateRepository: Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<StateModel>, u32)>, DomainError>;
    async fn find_by_stateid(&self, id: &Uuid) -> Result<Option<StateModel>, DomainError>;
    async fn insert(
        &self,
        state_create_model: &StateCreateModel,
    ) -> Result<StateModel, DomainError>;
    async fn update_by_stateid(
        &self,
        id: &Uuid,
        state_update_model: &StateUpdateModel,
    ) -> Result<StateModel, DomainError>;
    async fn delete_by_stateid(&self, id: &Uuid) -> Result<(), DomainError>;
}
