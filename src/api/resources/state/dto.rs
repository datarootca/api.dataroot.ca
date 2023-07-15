use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use validator::Validate;

use crate::{
    api::utils::validator::validate_page_size_max,
    domain::state::model::{StateCreateModel, StateModel, StateUpdateModel},
};

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateState {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 64))]
    pub extid: String,
    #[validate(length(max = 2))]
    pub symbol: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestCreateState> for StateCreateModel {
    fn from(value: RequestCreateState) -> Self {
        StateCreateModel::new(
            value.extid, 
            value.name,
            value.symbol,
            value.highres_link,
            value.photo_link,
            value.thumb_link
        )
    }
}
#[cfg(test)]
impl RequestCreateState {
    pub fn mock_default() -> Self {
        Self {
            name: "California".to_string(),
            symbol: "ca".to_string(),
            extid: "ca".to_string(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RequestUpdateState {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 2))]
    pub symbol: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestUpdateState> for StateUpdateModel {
    fn from(value: RequestUpdateState) -> Self {
        StateUpdateModel::new(
            value.name, 
            value.symbol,
            value.highres_link,
            value.photo_link,
            value.thumb_link,
        )
    }
}
#[cfg(test)]
impl RequestUpdateState {
    pub fn mock_default() -> Self {
        Self {
            name: "Czech republic".to_string(),
            symbol: "cs".to_string(),
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
pub struct RequestFindCategories {
    #[validate(length(max = 64))]
    pub name: Option<String>,
    pub page: Option<u32>,
    #[validate(custom = "validate_page_size_max")]
    pub page_size: Option<u32>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseState {
    pub stateid: i32,
    pub name: String,
    pub symbol: String,
    pub extid: String,
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
impl From<StateModel> for ResponseState {
    fn from(value: StateModel) -> Self {
        Self {
            stateid: value.stateid,
            name: value.name,
            symbol: value.symbol,
            extid: value.extid,
            highres_link: value.highres_link,
            thumb_link: value.thumb_link,
            photo_link: value.photo_link,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
