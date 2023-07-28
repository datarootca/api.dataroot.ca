use std::sync::Arc;



use crate::domain::{
    event::{
        model::{EventModel, EventUpdateModel},
        repository::EventRepository,
    },
    error::DomainError,
};

pub async fn execute(
    event_repository: Arc<dyn EventRepository>,
    id: i32,
    event_update_model: EventUpdateModel,
) -> Result<EventModel, DomainError> {
    let has_event = event_repository.find_by_eventid(&id).await?;
    if has_event.is_none() {
        return Err(DomainError::NotFound(String::from("Event id not found")));
    }

    let event = event_repository
        .update_by_eventid(&id, &event_update_model)
        .await?;

    Ok(event)
}

#[cfg(test)]
mod tests {
    use crate::{domain::event::model::{EventCreateModel, EventDetailModel}, api::utils::random_number, repository::event::{EventStatusOption, DateRangeOption}};

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
    async fn it_should_return_event_updated() {
        let mut event_repository = MockFakeEventRepository::new();

        let mock_event_model = EventModel::mock_default();
        let mut mock_request_event_update = EventUpdateModel::mock_default();
        mock_request_event_update.name = mock_event_model.name.clone();

        event_repository
            .expect_find_by_eventid()
            .return_once(|_| Ok(Some(mock_event_model)));

        event_repository
            .expect_update_by_eventid()
            .return_once(|_, _| Ok(EventModel::mock_default()));

        let response = execute(
            Arc::new(event_repository),
            random_number(),
            mock_request_event_update,
        )
        .await
        .unwrap();

        assert!(response.eventid != 0);
    }

    #[tokio::test]
    async fn it_should_return_error_not_found_event() {
        let mut event_repository = MockFakeEventRepository::new();
        event_repository
            .expect_find_by_eventid()
            .return_once(|_| Ok(None));

        let result = execute(
            Arc::new(event_repository),
            random_number(),
            EventUpdateModel::mock_default(),
        )
        .await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
