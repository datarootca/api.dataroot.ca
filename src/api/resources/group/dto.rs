use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[cfg(test)]
use crate::api::utils::random_string;

use crate::{
    api::utils::{validator::validate_page_size_max},
    domain::group::model::{GroupCreateModel, GroupModel, GroupUpdateModel, ImageLinks},
};

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateGroup {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 512))]
    pub description: String,
    #[validate(length(max = 100))]
    pub slug: String,
    #[validate(length(max = 64))]
    pub extid: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: Uuid,
    #[validate(length(max = 100))]
    pub organizer: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestCreateGroup> for GroupCreateModel {
    fn from(value: RequestCreateGroup) -> Self {
        GroupCreateModel::new(
            value.extid, 
            value.name,
            value.description,
            value.slug,
            value.active,
            value.private,
            value.members,
            value.cityid,
            value.organizer,
            ImageLinks{
                highres_link:   value.highres_link,
                photo_link: value.photo_link,
                thumb_link:  value.thumb_link
            },
        )
    }
}
#[cfg(test)]
impl RequestCreateGroup {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(10),
            description: "The Big Group".to_string(),
            extid: random_string(10),
            slug: random_string(10),
            organizer: "organizer".to_string(),
            active: true,
            private: true,
            members: 100,
            cityid: uuid::Uuid::new_v4(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RequestUpdateGroup {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 512))]
    pub description: String,
    #[validate(length(max = 100))]
    pub slug: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: Uuid,
    #[validate(length(max = 100))]
    pub organizer: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestUpdateGroup> for GroupUpdateModel {
    fn from(value: RequestUpdateGroup) -> Self {
        GroupUpdateModel::new(
            value.name,
            value.description,
            value.slug,
            value.active,
            value.private,
            value.members,
            value.cityid,
            value.organizer,
            ImageLinks{
                highres_link:   value.highres_link,
                photo_link: value.photo_link,
                thumb_link:  value.thumb_link
            },
        )
    }
}
#[cfg(test)]
impl RequestUpdateGroup {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(10),
            description: "The Big Group".to_string(),
            slug: random_string(10),
            organizer: "organizer".to_string(),
            active: true,
            private: true,
            members: 100,
            cityid: uuid::Uuid::new_v4(),
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
pub struct RequestFindGroup {
    #[validate(length(max = 64))]
    pub name: Option<String>,
    pub page: Option<u32>,
    #[validate(custom = "validate_page_size_max")]
    pub page_size: Option<u32>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseGroup {
    pub groupid: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub extid: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: Uuid,
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
