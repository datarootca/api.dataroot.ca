use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use validator::Validate;

use crate::{
    api::utils::{validator::validate_page_size_max},
    domain::article::model::{ ArticleModel},
};
#[derive(Debug, Clone, Deserialize, Validate, IntoParams)]
pub struct RequestFindArticle {
    #[validate(length(max = 64))]
    pub name: Option<String>,
    pub page: Option<u32>,
    #[validate(custom = "validate_page_size_max")]
    pub page_size: Option<u32>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseArticle {
    pub articleid: i32,
    pub extid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub time_m: i32,
    pub link: String,
    pub source: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highres_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_link: Option<String>,
    pub publish_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}
impl From<ArticleModel> for ResponseArticle {
    fn from(value: ArticleModel) -> Self {
        Self {
            articleid: value.articleid,
            extid: value.extid,
            name: value.name,
            description: value.description,
            time_m: value.time_m,
            link: value.link,
            source: value.source,
            author: value.author,
            highres_link: value.highres_link,
            photo_link: value.photo_link,
            thumb_link: value.thumb_link,
            publish_at: value.publish_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
