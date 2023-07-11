use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::utils::validator::validate_page_size_max,
    domain::article::model::{ArticleCreateModel, ArticleModel, ArticleUpdateModel},
};

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateArticle {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 512))]
    pub description: String,
    #[validate(length(max = 64))]
    pub source: String,
    #[validate(length(max = 64))]
    pub author: String,
    #[validate(length(max = 64))]
    pub link: String,
    #[validate(length(max = 64))]
    pub extid: String,
    pub stateid: Uuid,
    #[validate(length(max = 64))]
    pub slug: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestCreateArticle> for ArticleCreateModel {
    fn from(value: RequestCreateArticle) -> Self {
        ArticleCreateModel::new(
            value.name, 
            Some(value.description),
            value.extid,
            value.time_m,
            value.source,
            value.link,
            value.author,
            value.publish_at,
            value.highres_link,
            value.photo_link,
            value.thumb_link,
        )
    }
}
#[cfg(test)]
impl RequestCreateArticle {
    pub fn mock_default() -> Self {
        Self {
            name: "California".to_string(),
            slug: "ca".to_string(),
            extid: "ca".to_string(),
            stateid: uuid::Uuid::new_v4(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RequestUpdateArticle {
    #[validate(length(max = 64))]
    pub name: String,
    pub stateid: Uuid,
    #[validate(length(max = 64))]
    pub slug: String,
    #[validate(length(max = 512))]
    pub highres_link: Option<String>,
    #[validate(length(max = 512))]
    pub photo_link: Option<String>,
    #[validate(length(max = 512))]
    pub thumb_link: Option<String>,
}
impl From<RequestUpdateArticle> for ArticleUpdateModel {
    fn from(value: RequestUpdateArticle) -> Self {
        ArticleUpdateModel::new(
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
impl RequestUpdateArticle {
    pub fn mock_default() -> Self {
        Self {
            name: "California".to_string(),
            slug: "ca".to_string(),
            stateid: uuid::Uuid::new_v4(),
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
pub struct ResponseArticle {
    pub articleid: Uuid,
    pub stateid: Uuid,
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
impl From<ArticleModel> for ResponseArticle {
    fn from(value: ArticleModel) -> Self {
        Self {
            articleid: value.articleid,
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
