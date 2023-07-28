#[cfg(test)]
use crate::api::utils::random_number;
#[cfg(test)]
use crate::api::utils::random_string;
use crate::domain::city::model::CityModel;
use crate::domain::state::model::StateModel;

use chrono::{DateTime, Utc};
#[derive(Debug, Clone)]
pub struct ImageLinks {
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
#[derive(Debug, Clone)]
pub struct GroupCreateModel {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub extid: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: i32,
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
        cityid: i32,
        organizer: String,
        image: ImageLinks,
    ) -> Self {
        Self {
            extid,
            name,
            description,
            slug,
            active,
            private,
            members,
            cityid,
            organizer,
            highres_link: image.highres_link,
            photo_link: image.photo_link,
            thumb_link: image.thumb_link,
        }
    }
}

#[cfg(test)]
impl GroupCreateModel {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(10),
            description: "The Big Group".to_string(),
            extid: random_string(10),
            slug: random_string(10),
            organizer: "organizer".to_string(),
            active: false,
            private: true,
            members: 100,
            cityid: random_number(),
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
    pub cityid: i32,
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
        cityid: i32,
        organizer: String,
        image: ImageLinks,
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
            highres_link: image.highres_link,
            photo_link: image.photo_link,
            thumb_link: image.thumb_link,
        }
    }
}
#[cfg(test)]
impl GroupUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(10),
            description: "The Big Group".to_string(),
            slug: random_string(10),
            organizer: "organizer".to_string(),
            active: true,
            private: true,
            members: 100,
            cityid: random_number(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

pub struct DetailedGroup {
    pub group: GroupModel,
    pub city: CityModel,
    pub state: StateModel,
}

impl DetailedGroup {
    pub fn new(group: GroupModel, city: CityModel, state: StateModel) -> Self {
        Self { group, city, state }
    }
}


#[derive(Debug, Clone)]
pub struct GroupPageModel {
    pub group_name: String,
    pub group_slug: String,
    pub group_highres_link: Option<String>,
    pub group_photo_link: Option<String>,
    pub group_thumb_link: Option<String>,
    pub state_symbol: Option<String>,
    pub city_name: Option<String>,
    pub city_slug: Option<String>,
    pub organizer: String,
    pub event_count: i64,
    pub members: i32,
}

#[cfg(test)]
impl GroupPageModel {
    pub fn mock_default() -> Self {
        Self {
            group_name: random_string(10),
            city_name: Some("The Big Group".to_string()),
            state_symbol: Some(random_string(10)),
            city_slug: Some(random_string(10)),
            organizer: "organizer"  .to_string(),
            event_count: 100,
            members: random_number(),
            group_slug: random_string(10),
            group_highres_link: Some("group_highres_link".to_string()),
            group_photo_link: Some("group_photo_link".to_string()),
            group_thumb_link: Some("group_thumb_link".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupModel {
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
            groupid: random_number(),
            name: random_string(10),
            description: "The Big Group".to_string(),
            extid: random_string(10),
            slug: random_string(10),
            organizer: "organizer"  .to_string(),
            active: true,
            private: true,
            members: 100,
            cityid: random_number(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
