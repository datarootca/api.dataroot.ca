use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CityCreateModel {
    pub cityid: Uuid,
    pub name: String,
    pub slug: String,
    pub stateid: Uuid,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub extid: String,
}
impl CityCreateModel {
    pub fn new(
        name: String, 
        slug: String,
        stateid: Uuid,
        extid: String,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link:Option<String>,
    ) -> Self {
        Self {
            cityid: Uuid::new_v4(),
            name,
            slug,
            stateid,
            extid,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}

#[cfg(test)]
impl CityCreateModel {
    pub fn mock_default() -> Self {
        Self {
            stateid: Uuid::new_v4(),
            cityid: Uuid::new_v4(),
            name: "New Jersey".to_string(),
            slug: "new-jersey".to_string(),
            extid: "test".to_string(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CityUpdateModel {
    pub name: String,
    pub slug: String,
    pub stateid: Uuid,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl CityUpdateModel {
    pub fn new(
        name: String, 
        slug: String,
        stateid: Uuid,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link:Option<String>,
    ) -> Self {
        Self {
            name,
            slug,
            stateid,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl CityUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "Ohio 1".to_string(),
            slug: "ohio 1".to_string(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            stateid: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CityModel {
    pub cityid: Uuid,
    pub name: String,
    pub slug: String,
    pub stateid: Uuid,
    pub extid: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
impl CityModel {
    pub fn mock_default() -> Self {
        Self {
            cityid: Uuid::new_v4(),
            name: "Ohio".to_string(),
            slug: "ohio".to_string(),
            extid: "ca".to_string(),
            stateid: Uuid::new_v4(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
