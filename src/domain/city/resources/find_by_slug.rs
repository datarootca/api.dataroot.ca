use std::sync::Arc;



use crate::domain::{
    city::{model::{CityDetailModel}, repository::CityRepository},
    error::DomainError,
};

pub async fn execute(
    city_repository: Arc<dyn CityRepository>,
    slug: String,
) -> Result<Option<CityDetailModel>, DomainError> {
    if let Some(city) = city_repository.find_by_slug(slug.to_owned()).await? {
        return Ok(Some(city));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::{domain::city::model::{CityCreateModel, CityUpdateModel,CityDetailModel,CityModel}, api::utils::{random_string}};

    use super::*;

    mock! {
        pub FakeCityRepository { }

        #[async_trait]
        impl CityRepository for FakeCityRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<CityModel>, u32)>, DomainError>;
            async fn find_by_slug(&self, slug: String) -> Result<Option<CityDetailModel>, DomainError>;
            async fn find_by_cityid(&self, id: &i32) -> Result<Option<CityModel>, DomainError>;
            async fn insert(&self,city_create_model: &CityCreateModel) -> Result<CityModel, DomainError>;
            async fn update_by_cityid(&self,id: &i32,city_update_model: &CityUpdateModel) -> Result<CityModel, DomainError>;
            async fn delete_by_cityid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_city_finded() {
        let mut city_repository = MockFakeCityRepository::new();

        city_repository
            .expect_find_by_slug()
            .return_once(|_| Ok(Some(CityDetailModel::mock_default())));

        let result = execute(Arc::new(city_repository), random_string(10)).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_category() {
        let mut city_repository = MockFakeCityRepository::new();

        city_repository
            .expect_find_by_slug()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(city_repository), random_string(10)).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}