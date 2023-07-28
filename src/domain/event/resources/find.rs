use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::{domain::{
    event::{model::{EventDetailModel}, repository::EventRepository},
    error::DomainError,
}, repository::event::{DateRangeOption, EventStatusOption}};

pub async fn execute(
    event_repository: Arc<dyn EventRepository>,
    name: Option<String>,
    in_person: Option<bool>,
    is_online: Option<bool>,
    group_slug: Option<String>,
    location: Option<String>,
    has_fee: Option<bool>,
    rsvp_limit: Option<u32>,
    status: Option<EventStatusOption>,
    time_frame: Option<DateRangeOption>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<EventDetailModel>, u32)>, DomainError> {
    let event = event_repository.find(
        &name,
        &in_person,
        &is_online,
        &group_slug,
        &location,
        &has_fee,
        &rsvp_limit,
        &status,
        &time_frame,
        &start_date,
        &end_date,
        &page,
        &page_size,
    ).await?;

    if event.is_some() {
        return Ok(event);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::{domain::event::model::{EventCreateModel, EventUpdateModel, EventDetailModel, EventModel}, repository::event::{EventStatusOption, DateRangeOption}};

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
    async fn it_should_return_event_finded() {
        let mut event_repository = MockFakeEventRepository::new();

        event_repository
            .expect_find()
            .return_once(|_, _, _,_, _,_,_,_,_, _, _,_,_| Ok(Some((vec![EventDetailModel::mock_default()], 1))));

        let (event, count) = execute(
            Arc::new(event_repository), 
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None,
        None, 
        None, 
        None, 
        None, 
        1, 
        12
        )
            .await
            .unwrap()
            .unwrap();

        assert!(!event.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut event_repository = MockFakeEventRepository::new();
        event_repository
            .expect_find()
            .return_once(|_, _, _, _, _,_, _, _,_, _, _,_,_| Ok(None));

        let response = execute(
            Arc::new(event_repository), 
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None,
        None, 
        None, 
        None, 
        None, 
        1, 
        12,
    )
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
