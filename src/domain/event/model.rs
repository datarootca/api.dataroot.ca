use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EventCreateModel {
    pub eventid: Uuid,
    pub name: String,
    pub description: String,
    pub location: String,
    pub extid: String,
    pub groupid: Uuid,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: i32,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl EventCreateModel {
    pub fn new(
        name: String, 
        description: String,
        location: String,
        groupid: Uuid,
        extid: String,
        link: String,
        in_person: bool,
        is_online: bool,
        duration: i32,
        waitlist_count: i32,
        yes_rsvp_count: i32,
        fee: bool,
        rsvp_limit: i32,
        time: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            eventid: uuid::Uuid::new_v4(),
            name,
            description,
            location, 
            groupid,
            extid, 
            in_person, 
            is_online, 
            time, 
            duration, 
            link, 
            waitlist_count, 
            fee, 
            yes_rsvp_count, 
            rsvp_limit, 
            highres_link, 
            photo_link,
            thumb_link,
        }
    }
}

#[cfg(test)]
impl EventCreateModel {
    pub fn mock_default() -> Self {
        Self {
            eventid: uuid::Uuid::new_v4(),
            name: "Event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            groupid: uuid::Uuid::new_v4(),
            extid: "m-event".to_string(),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: "".to_string(),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: 5,
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventUpdateModel {
    pub name: String,
    pub description: String,
    pub location: String,
    pub groupid: Uuid,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: i32,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl EventUpdateModel {
    pub fn new(
        name: String, 
        description: String,
        location: String,
        groupid: Uuid,
        link: String,
        in_person: bool,
        is_online: bool,
        duration: i32,
        waitlist_count: i32,
        yes_rsvp_count: i32,
        fee: bool,
        rsvp_limit: i32,
        time: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            location, 
            groupid,
            in_person, 
            is_online, 
            time, 
            duration, 
            link, 
            waitlist_count, 
            fee, 
            yes_rsvp_count, 
            rsvp_limit, 
            highres_link, 
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl EventUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "Event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            groupid: uuid::Uuid::new_v4(),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: "".to_string(),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: 5,
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventModel {
    pub eventid: Uuid,
    pub name: String,
    pub description: String,
    pub location: String,
    pub extid: String,
    pub groupid: Uuid,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: i32,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[cfg(test)]
impl EventModel {
    pub fn mock_default() -> Self {
        Self {
            eventid: uuid::Uuid::new_v4(),
            groupid: uuid::Uuid::new_v4(),
            name: "Event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            extid: "m-event".to_string(),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: "".to_string(),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: 5,
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}
