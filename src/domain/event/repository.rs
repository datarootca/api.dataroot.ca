use async_trait::async_trait;
use chrono::{Utc, DateTime};


use crate::{domain::error::DomainError, repository::event::{EventStatusOption, DateRangeOption}};

use super::model::{EventCreateModel, EventModel, EventUpdateModel, EventDetailModel};

#[async_trait]
pub trait EventRepository: Send + Sync {
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
    async fn insert(
        &self,
        event_create_model: &EventCreateModel,
    ) -> Result<EventModel, DomainError>;
    async fn update_by_eventid(
        &self,
        id: &i32,
        event_update_model: &EventUpdateModel,
    ) -> Result<EventModel, DomainError>;
    async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError>;
}
