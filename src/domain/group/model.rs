use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GroupCreateModel {
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
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl GroupCreateModel {
    pub fn new(
        extid: String,
        name: String,
        description: String,
        slug: String,
        active: bool,
        private: bool,
        members: i32,
        cityid: Uuid,
        organizer: String,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            groupid: Uuid::new_v4(),
            extid,
            name,
            description,
            slug,
            active,
            private,
            members,
            cityid,
            organizer,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}

#[cfg(test)]
impl GroupCreateModel {
    pub fn mock_default() -> Self {
        Self {
            groupid: uuid::Uuid::new_v4(),
            name: "Group".to_string(),
            description: "The Big Group".to_string(),
            extid: "group".to_string(),
            slug: "the-big-group".to_string(),
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

#[derive(Debug, Clone)]
pub struct GroupUpdateModel {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: Uuid,
    pub organizer: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl GroupUpdateModel {
    pub fn new(
        name: String,
        description: String,
        slug: String,
        active: bool,
        private: bool,
        members: i32,
        cityid: Uuid,
        organizer: String,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            slug,
            active,
            private,
            members,
            cityid,
            organizer,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl GroupUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "Group".to_string(),
            description: "The Big Group".to_string(),
            slug: "the-big-group".to_string(),
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

#[derive(Debug, Clone)]
pub struct GroupModel {
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
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
impl GroupModel {
    pub fn mock_default() -> Self {
        Self {
            groupid: uuid::Uuid::new_v4(),
            name: "Group".to_string(),
            description: "The Big Group".to_string(),
            extid: "group".to_string(),
            slug: "the-big-group".to_string(),
            organizer: "organizer"  .to_string(),
            active: true,
            private: true,
            members: 100,
            cityid: uuid::Uuid::new_v4(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
