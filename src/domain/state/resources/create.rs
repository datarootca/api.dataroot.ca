use std::sync::Arc;

use crate::domain::state::model::StateModel;
use crate::domain::{
    state::{model::StateCreateModel, repository::StateRepository},
    error::DomainError,
};

pub async fn execute(
    state_repository: Arc<dyn StateRepository>,
    state_create_model: StateCreateModel,
) -> Result<StateModel, DomainError> {
    let state = state_repository.insert(&state_create_model).await?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use crate::domain::state::model::StateUpdateModel;

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    use uuid::Uuid;

    mock! {
        pub FakeStateRepository { }

        #[async_trait]
        impl StateRepository for FakeStateRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<StateModel>, u32)>, DomainError>;
            async fn find_by_stateid(&self, id: &Uuid) -> Result<Option<StateModel>, DomainError>;
            async fn insert(&self,state_create_model: &StateCreateModel) -> Result<StateModel, DomainError>;
            async fn update_by_stateid(&self,id: &Uuid,state_update_model: &StateUpdateModel) -> Result<StateModel, DomainError>;
            async fn delete_by_stateid(&self, id: &Uuid) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_state_created() {
        let mut state_repository = MockFakeStateRepository::new();

        state_repository
            .expect_insert()
            .return_once(|_| Ok(StateModel::mock_default()));

        let result = execute(
            Arc::new(state_repository),
            StateCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
