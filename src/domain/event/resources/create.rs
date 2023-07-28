use std::sync::Arc;

use crate::domain::event::model::EventModel;
use crate::domain::{
    event::{model::EventCreateModel, repository::EventRepository},
    error::DomainError,
};

pub async fn execute(
    event_repository: Arc<dyn EventRepository>,
    event_create_model: EventCreateModel,
) -> Result<EventModel, DomainError> {
    let event = event_repository.insert(&event_create_model).await?;
    Ok(event)
}

#[cfg(test)]
mod tests {
    use crate::{domain::event::model::{EventUpdateModel, EventDetailModel}, repository::event::{EventStatusOption, DateRangeOption}};

    use super::*;

    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use mockall::mock;
    

    mock! {
        pub FakeEventRepository { }

        #[async_trait]
        impl EventRepository for FakeEventRepository {
            async fn find(
        &self,
        name: &Option<String>,
        in_person: &Option<bool>,
        is_online: &Option<bool>,
        group_slug: &Option<String>,
        location: &Option<String>,
        has_fee: &Option<bool>,
        rsvp_limit: &Option<u32>,
        status: &Option<EventStatusOption>,
        time_frame: &Option<DateRangeOption>,
        start_date: &Option<DateTime<Utc>>,
        end_date: &Option<DateTime<Utc>>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<EventDetailModel>, u32)>, DomainError>;
            async fn find_by_eventid(&self, id: &i32) -> Result<Option<EventModel>, DomainError>;
            async fn insert(&self,event_create_model: &EventCreateModel) -> Result<EventModel, DomainError>;
            async fn update_by_eventid(&self,id: &i32,event_update_model: &EventUpdateModel) -> Result<EventModel, DomainError>;
            async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_event_created() {
        let mut event_repository = MockFakeEventRepository::new();

        event_repository
            .expect_insert()
            .return_once(|_| Ok(EventModel::mock_default()));

        let result = execute(
            Arc::new(event_repository),
            EventCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
