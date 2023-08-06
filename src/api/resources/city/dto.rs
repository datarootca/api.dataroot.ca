use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use validator::Validate;


use crate::{
    api::utils::{validator::validate_page_size_max},
    domain::city::model::{ CityModel, CityDetailModel},
};


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

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseDetailCity {
    pub name: String,
    pub slug: String,
    pub state_symbol: String,
    pub state_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highres_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_link: Option<String>,
}
impl From<CityDetailModel> for ResponseDetailCity {
    fn from(value: CityDetailModel) -> Self {
        Self {
            name: value.name,
            slug: value.slug,
            state_name: value.state_name,
            state_symbol: value.state_symbol,
            highres_link: value.highres_link,
            thumb_link: value.thumb_link,
            photo_link: value.photo_link,
        }
    }
}

