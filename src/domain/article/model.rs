use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ArticleCreateModel {
    pub articleid: Uuid,
    pub extid: String,
    pub name: String,
    pub description: Option<String>,
    pub time_m: i32,
    pub source: String,
    pub link: String,
    pub author: String,
    pub publish_at: DateTime<Utc>,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl ArticleCreateModel {
    pub fn new(
        name: String,
        description: Option<String>,
        extid: String,
        time_m: i32,
        source: String,
        link: String,
        author: String,
        publish_at: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            articleid: Uuid::new_v4(),
            name,
            description,
            extid,
            time_m,
            source,
            author,
            link,
            publish_at,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}

#[cfg(test)]
impl ArticleCreateModel {
    pub fn mock_default() -> Self {
        Self {
            articleid: uuid::Uuid::new_v4(),
            extid: "1".to_string(),
            name: "article".to_string(),
            link: "article".to_string(),
            description: Some("The famous article".to_string()),
            time_m: 5,
            source: "source".to_string(),
            author: "author".to_string(),
            publish_at: DateTime::default(),
            highres_link: Some("The img".to_string()),
            photo_link: Some("The img".to_string()),
            thumb_link: Some("The img".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArticleUpdateModel {
    pub name: String,
    pub description: Option<String>,
    pub time_m: i32,
    pub source: String,
    pub link: String,
    pub author: String,
    pub publish_at: DateTime<Utc>,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl ArticleUpdateModel {
    pub fn new(
        name: String,
        description: Option<String>,
        time_m: i32,
        link: String,
        source: String,
        author: String,
        publish_at: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            time_m,
            link,
            source,
            author,
            publish_at,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl ArticleUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "article".to_string(),
            description: Some("The famous article".to_string()),
            time_m: 5,
            source: "source".to_string(),
            link: "link".to_string(),
            author: "author".to_string(),
            publish_at: DateTime::default(),
            highres_link: Some("The img".to_string()),
            photo_link: Some("The img".to_string()),
            thumb_link: Some("The img".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArticleModel {
    pub articleid: Uuid,
    pub extid: String,
    pub name: String,
    pub description: Option<String>,
    pub time_m: i32,
    pub link: String,
    pub source: String,
    pub author: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub publish_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
impl ArticleModel {
    pub fn mock_default() -> Self {
        Self {
            articleid: uuid::Uuid::new_v4(),
            extid: "1".to_string(),
            name: "article".to_string(),
            description: Some("The famous article".to_string()),
            link: "hyperlink".to_string(),
            time_m: 5,
            source: "source".to_string(),
            author: "author".to_string(),
            highres_link: Some("highres_link".to_string()),
            photo_link: Some("photo_link".to_string()),
            thumb_link: Some("thumb_link".to_string()),
            publish_at: DateTime::default(),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
