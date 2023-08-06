use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use validator::Validate;

use crate::{
    api::utils::{validator::validate_page_size_max},
    domain::group::model::{ GroupModel, GroupPageModel, DetailedGroup},
};

#[derive(Debug, Clone, Deserialize, Validate, IntoParams)]
pub struct RequestFindGroup {
    #[validate(length(max = 64))]
    pub city: Option<String>,
    #[validate(length(max = 64))]
    pub name: Option<String>,
    pub page: Option<u32>,
    #[validate(custom = "validate_page_size_max")]
    pub page_size: Option<u32>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseGroup {
    pub groupid: i32,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub extid: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: i32,
    pub organizer: String,
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
impl From<GroupModel> for ResponseGroup {
    fn from(value: GroupModel) -> Self {
        Self {
            groupid: value.groupid,
            name: value.name,
            description: value.description,
            slug: value.slug,
            extid: value.extid,
            active: value.active,
            private: value.private,
            members: value.members,
            cityid: value.cityid,
            organizer: value.organizer,
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
pub struct ResponsePageGroup {
    pub group_name: String,
    pub group_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_highres_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_photo_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_thumb_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_slug: Option<String>,
    pub organizer: String,
    pub event_count: i64,
    pub members: i32,
}
impl From<GroupPageModel> for ResponsePageGroup {
    fn from(value: GroupPageModel) -> Self {
        Self {
            group_name: value.group_name,
            group_slug: value.group_slug,
            group_highres_link: value.group_highres_link,
            group_photo_link: value.group_photo_link,
            group_thumb_link: value.group_thumb_link,
            state_symbol: value.state_symbol,
            city_name: value.city_name,
            city_slug: value.city_slug,
            organizer: value.organizer,
            event_count: value.event_count,
            members: value.members,
        }
    }
}


#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponseDetailGroup {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub extid: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub city_name: String,
    pub state_symbol: String,
    pub organizer: String,
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

impl From<DetailedGroup> for ApiResponseDetailGroup {
    fn from(value: DetailedGroup) -> Self {
        Self {
            name: value.group.name,
            description: value.group.description,
            slug: value.group.slug,
            extid: value.group.extid,
            active: value.group.active,
            private: value.group.private,
            members: value.group.members,
            city_name: value.city.name,
            state_symbol: value.state.symbol,
            organizer: value.group.organizer,
            highres_link: value.group.highres_link,
            photo_link: value.group.photo_link,
            thumb_link: value.group.thumb_link,
            created_at: value.group.created_at,
            updated_at: value.group.updated_at,
        }
    }
}
