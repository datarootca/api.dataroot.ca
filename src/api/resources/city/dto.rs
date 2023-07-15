use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use validator::Validate;

#[cfg(test)]
use crate::api::utils::{
    random_string,
    random_number,
};

use crate::{
    api::utils::{validator::validate_page_size_max},
    domain::city::model::{CityCreateModel, CityModel, CityUpdateModel},
};

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateCity {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 64))]
    pub extid: String,
    pub stateid: i32,
    #[validate(length(max = 64))]
    pub slug: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestCreateCity> for CityCreateModel {
    fn from(value: RequestCreateCity) -> Self {
        CityCreateModel::new(
            value.name, 
            value.slug,
            value.stateid,
            value.extid,
            value.highres_link,
            value.photo_link,
            value.thumb_link
        )
    }
}
#[cfg(test)]
impl RequestCreateCity {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(10),
            slug: random_string(10),
            extid: random_string(10),
            stateid: random_number(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RequestUpdateCity {
    #[validate(length(max = 64))]
    pub name: String,
    pub stateid: i32,
    #[validate(length(max = 64))]
    pub slug: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestUpdateCity> for CityUpdateModel {
    fn from(value: RequestUpdateCity) -> Self {
        CityUpdateModel::new(
            value.name, 
            value.slug,
            value.stateid,
            value.highres_link,
            value.photo_link,
            value.thumb_link,
        )
    }
}
#[cfg(test)]
impl RequestUpdateCity {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(20),
            slug: random_string(2),
            stateid: random_number(),
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
pub struct ResponseCity {
    pub cityid: i32,
    pub stateid: i32,
    pub name: String,
    pub slug: String,
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
impl From<CityModel> for ResponseCity {
    fn from(value: CityModel) -> Self {
        Self {
            cityid: value.cityid,
            stateid: value.stateid,
            name: value.name,
            slug: value.slug,
            extid: value.extid,
            highres_link: value.highres_link,
            thumb_link: value.thumb_link,
            photo_link: value.photo_link,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
