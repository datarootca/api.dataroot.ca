use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::utils::validator::validate_page_size_max,
    domain::event::model::{EventCreateModel, EventModel, EventUpdateModel},
};

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateEvent {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 512))]
    pub description: String,
    #[validate(length(max = 64))]
    pub location: String,
    #[validate(length(max = 64))]
    pub extid: String,
    pub groupid: Uuid,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    #[validate(length(max = 64))]
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: i32,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestCreateEvent> for EventCreateModel {
    fn from(value: RequestCreateEvent) -> Self {
        EventCreateModel::new(
            value.name, 
            value.description,
            value.location,
            value.groupid,
            value.extid,
            value.link,
            value.in_person,
            value.is_online,
            value.duration,
            value.waitlist_count,
            value.yes_rsvp_count,
            value.fee,
            value.rsvp_limit,
            value.time,
            value.highres_link,
            value.photo_link,
            value.thumb_link,
        )
    }
}
#[cfg(test)]
impl RequestCreateEvent {
    pub fn mock_default() -> Self {
        Self {
            name: "Event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            groupid: uuid::Uuid::new_v4(),
            extid: "m-event".to_string(),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: "".to_string(),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: 5,
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RequestUpdateEvent {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 512))]
    pub description: String,
    #[validate(length(max = 64))]
    pub location: String,
    pub groupid: Uuid,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    #[validate(length(max = 64))]
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: i32,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestUpdateEvent> for EventUpdateModel {
    fn from(value: RequestUpdateEvent) -> Self {
        EventUpdateModel::new(
            value.name, 
            value.description,
            value.location,
            value.groupid,
            value.link,
            value.in_person,
            value.is_online,
            value.duration,
            value.waitlist_count,
            value.yes_rsvp_count,
            value.fee,
            value.rsvp_limit,
            value.time,
            value.highres_link,
            value.photo_link,
            value.thumb_link,
        )
    }
}
#[cfg(test)]
impl RequestUpdateEvent {
    pub fn mock_default() -> Self {
        Self {
            name: "Event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            groupid: uuid::Uuid::new_v4(),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: "".to_string(),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: 5,
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[derive(Debug, Clone, Deserialize, Validate, IntoParams)]
pub struct RequestFindEvent {
    #[validate(length(max = 64))]
    pub name: Option<String>,
    pub page: Option<u32>,
    #[validate(custom = "validate_page_size_max")]
    pub page_size: Option<u32>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseEvent {
    pub eventid: Uuid,
    pub name: String,
    pub description: String,
    pub location: String,
    pub extid: String,
    pub groupid: Uuid,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highres_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}
impl From<EventModel> for ResponseEvent {
    fn from(value: EventModel) -> Self {
        Self {
            eventid: value.eventid,
            name: value.name,
            description: value.description,
            location: value.location,
            extid: value.extid,
            groupid: value.groupid,
            in_person: value.in_person,
            is_online: value.is_online,
            time: value.time,
            duration: value.duration,
            link: value.link,
            waitlist_count: value.waitlist_count,
            fee: value.fee,
            yes_rsvp_count: value.yes_rsvp_count,
            rsvp_limit: value.rsvp_limit,
            highres_link: value.highres_link,
            photo_link: value.photo_link,
            thumb_link: value.thumb_link,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
