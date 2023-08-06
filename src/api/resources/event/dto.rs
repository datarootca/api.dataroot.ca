use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use validator::Validate;

use crate::{
    api::utils::{validator::{validate_page_size_max,validate_event_status_option,validate_event_request}},
    domain::event::model::{ EventModel, EventDetailModel}, repository::event::{EventStatusOption, DateRangeOption},
};


#[derive(Debug, Clone, Deserialize, Validate, IntoParams)]
#[validate(schema(function = "validate_event_request"))]
pub struct RequestFindEvent {
    #[validate(length(max = 64))]
    pub name: Option<String>,
    pub in_person: Option<bool>,
    pub is_online: Option<bool>,
    #[validate(length(max = 64))]
    pub group_slug: Option<String>,
    #[validate(length(max = 64))]
    pub location: Option<String>,
    pub has_fee: Option<bool>,
    pub rsvp_limit: Option<u32>,
    pub time_frame: Option<DateRangeOption>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    #[validate(custom = "validate_event_status_option")]
    pub status: Option<EventStatusOption>,
    pub page: Option<u32>,
    #[validate(custom = "validate_page_size_max")]
    pub page_size: Option<u32>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseEvent {
    pub eventid: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    pub extid: String,
    pub groupid: i32,
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

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseDetailEvent {
    pub eventid: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_slug: Option<String>,
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
impl From<EventDetailModel> for ResponseDetailEvent {
    fn from(value: EventDetailModel) -> Self {
        Self {
            eventid: value.eventid,
            name: value.name,
            description: value.description,
            location: value.location,
            group_slug: value.group_slug,
            group_name: value.group_name,
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