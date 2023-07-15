use std::sync::Arc;



use crate::domain::{
    categories::{
        model::{CategoryModel, CategoryUpdateModel},
        repository::CategoryRepository,
    },
    error::DomainError,
};

pub async fn execute(
    category_repository: Arc<dyn CategoryRepository>,
    id: i32,
    category_update_model: CategoryUpdateModel,
) -> Result<CategoryModel, DomainError> {
    let has_category = category_repository.find_by_id(&id).await?;
    if has_category.is_none() {
        return Err(DomainError::NotFound(String::from("Category id not found")));
    }

    let category = category_repository
        .update_by_id(&id, &category_update_model)
        .await?;

    Ok(category)
}

#[cfg(test)]
mod tests {
    use crate::{domain::categories::model::CategoryCreateModel, api::utils::random_number};

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        pub FakeCategoryRepository { }

        #[async_trait]
        impl CategoryRepository for FakeCategoryRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<CategoryModel>, u32)>, DomainError>;
            async fn find_by_id(&self, id: &i32) -> Result<Option<CategoryModel>, DomainError>;
            async fn insert(&self,category_create_model: &CategoryCreateModel) -> Result<CategoryModel, DomainError>;
            async fn update_by_id(&self,id: &i32,category_update_model: &CategoryUpdateModel) -> Result<CategoryModel, DomainError>;
            async fn delete_by_id(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_category_updated() {
        let mut category_repository = MockFakeCategoryRepository::new();

        let mock_category_model = CategoryModel::mock_default();
        let mut mock_request_category_update = CategoryUpdateModel::mock_default();
        mock_request_category_update.name = mock_category_model.name.clone();

        category_repository
            .expect_find_by_id()
            .return_once(|_| Ok(Some(mock_category_model)));

        category_repository
            .expect_update_by_id()
            .return_once(|_, _| Ok(CategoryModel::mock_default()));

        let response = execute(
            Arc::new(category_repository),
            random_number(),
            mock_request_category_update,
        )
        .await
        .unwrap();

        assert!(response.id != 0);
    }

    #[tokio::test]
    async fn it_should_return_error_not_found_category() {
        let mut category_repository = MockFakeCategoryRepository::new();
        category_repository
            .expect_find_by_id()
            .return_once(|_| Ok(None));

        let result = execute(
            Arc::new(category_repository),
            random_number(),
            CategoryUpdateModel::mock_default(),
        )
        .await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
