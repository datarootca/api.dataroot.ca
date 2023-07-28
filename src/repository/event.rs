use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;

use serde::{Serialize, Deserialize};
use tokio_postgres::{types::ToSql, Row};

use crate::domain::{
    event::{
        model::{EventCreateModel, EventModel, EventUpdateModel, EventDetailModel},
        repository::EventRepository,
    },
    error::DomainError,
};

const QUERY_FIND_EVENT: &str = "
    select
        e.eventid,
        e.name,
        e.description,
        g.slug as group_slug,
        g.name as group_name,
        e.extid,
        e.location,
        e.groupid,
        e.in_person,
        e.time,
        e.duration,
        e.link,
        e.waitlist_count,
        e.is_online,
        e.yes_rsvp_count,
        e.fee,
        e.created_at,
        e.updated_at,
        e.highres_link,
        e.photo_link,
        e.thumb_link,
        e.rsvp_limit,
        count(1) over ()::OID as count
    from
        event e
    LEFT JOIN \"group\" g using(groupid)";

const QUERY_FIND_EVENT_BY_ID: &str = "
    select
        e.eventid,
        e.name,
        e.description,
        e.extid,
        e.location,
        e.groupid,
        e.in_person,
        e.time,
        e.duration,
        e.link,
        e.waitlist_count,
        e.is_online,
        e.yes_rsvp_count,
        e.fee,
        e.created_at,
        e.updated_at,
        e.highres_link,
        e.rsvp_limit,
        e.photo_link,
        e.thumb_link,
        count(1) over ()::OID as count
    from
        event e
    where 
        eventid = $1;";

const QUERY_INSERT_EVENT: &str = "
    insert into event(name,description,extid,location,groupid,in_person,time,duration,link,waitlist_count,is_online,yes_rsvp_count,fee,highres_link,photo_link,thumb_link,rsvp_limit)
    values
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)
    returning
        eventid,
        name,
        description,
        extid,
        location,
        groupid,
        in_person,
        time,
        duration,
        link,
        waitlist_count,
        is_online,
        yes_rsvp_count,
        rsvp_limit,
        fee,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link;";

const QUERY_UPDATE_EVENT_BY_ID: &str = "
    update
        event 
    set
        name=$2,
        description=$3,
        location=$4,
        groupid=$5,
        in_person=$6,
        time=$7,
        duration=$8,
        link=$9,
        waitlist_count=$10,
        is_online=$11,
        yes_rsvp_count=$12,
        fee=$13,
        highres_link=$14,
        photo_link=$15,
        thumb_link=$16,
        rsvp_limit=$17,
        updated_at=now()
    where
        eventid = $1
    returning
        eventid,
        name,
        description,
        extid,
        location,
        groupid,
        in_person,
        time,
        duration,
        link,
        waitlist_count,
        is_online,
        yes_rsvp_count,
        rsvp_limit,
        fee,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link;";

const QUERY_DELETE_EVENT_BY_ID: &str = "
            delete from
                event 
            where
                eventid = $1;";

pub struct PgEventRepository {
    pool: Arc<Pool>,
}
impl PgEventRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum EventStatusOption {
    Upcomming,
    Recurrent,
    Past,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum DateRangeOption {
    Today,
    ThisWeek,
    ThisMonth,
    Custom,
}

#[async_trait]
impl EventRepository for PgEventRepository {
    async fn find(
        &self,
        name: &Option<String>,
        in_person: &Option<bool>,
        is_online: &Option<bool>,
        group_slug: &Option<String>,
        location: &Option<String>,
        has_fee: &Option<bool>,
        rsvp_limit: &Option<u32>,
        status: &Option<EventStatusOption>,
        time_frame: &Option<DateRangeOption>,
        start_date: &Option<DateTime<Utc>>,
        end_date: &Option<DateTime<Utc>>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<EventDetailModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "e.name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }

        if let Some(in_person) = in_person {
            queries.push(format!(
                "e.in_person = ${}",
                params.len() + 1
            ));
            params.push(in_person);
        }

        if let Some(is_online) = is_online {
            queries.push(format!(
                "e.is_online = ${}",
                params.len() + 1
            ));
            params.push(is_online);
        }

        if let Some(group_slug) = group_slug {
            queries.push(format!(
                "EXISTS(SELECT 1 FROM \"group\" g WHERE e.groupid=\"g\".groupid and \"g\".slug = ${})",
                params.len() + 1
            ));
            params.push(group_slug);
        }

        if let Some(location) = location {
            queries.push(format!(
                "e.location = ${}",
                params.len() + 1
            ));
            params.push(location);
        }

        if let Some(has_fee) = has_fee {
            queries.push(format!(
                "e.fee = ${}",
                params.len() + 1
            ));
            params.push(has_fee);
        }

        if let Some(rsvp_limit) = rsvp_limit {
            queries.push(format!(
                "e.rsvp_limit = ${}",
                params.len() + 1
            ));
            params.push(rsvp_limit);
        }

        if let Some(status) = status {
            let status_query = match status {
                EventStatusOption::Upcomming => "e.time > NOW()",
                EventStatusOption::Past => "(e.time + INTERVAL '1 second' * e.duration) <= NOW()",
                EventStatusOption::Recurrent => "(e.time <= NOW() AND (e.time + INTERVAL '1 second' * e.duration) > NOW())",
            };
            queries.push(status_query.to_string());
        }

        if let Some(time_frame) = time_frame {
            let time_frame_query = match time_frame {
                DateRangeOption::Today => "e.time >= CURRENT_DATE AND e.time < CURRENT_DATE + INTERVAL '1 day'".to_string(),
                DateRangeOption::ThisWeek => "e.time >= DATE_TRUNC('week', CURRENT_DATE) AND e.time < DATE_TRUNC('week', CURRENT_DATE) + INTERVAL '1 week'".to_string(),
                DateRangeOption::ThisMonth => "e.time >= DATE_TRUNC('month', CURRENT_DATE) AND e.time < DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month'".to_string(),
                DateRangeOption::Custom => {
                    let query_str = format!(
                        "e.time >= ${} AND e.time <= ${}",
                        params.len() + 1,
                        params.len() + 2
                    );
                    if let Some(start_date) = start_date {
                        params.push(start_date);
                    }
                    if let Some(end_date) = end_date {
                        params.push(end_date);
                    }
                    query_str
                },
            };
            if !time_frame_query.is_empty() {
                queries.push(time_frame_query);
            }
        }

        let mut query = String::from(QUERY_FIND_EVENT);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let events: Vec<EventDetailModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((events, count)));
        }

        return Ok(None);
    }

    async fn find_by_eventid(&self, id: &i32) -> Result<Option<EventModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_EVENT_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        event_create_model: &EventCreateModel,
    ) -> Result<EventModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_EVENT).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &event_create_model.name,
                    &event_create_model.description,
                    &event_create_model.extid,
                    &event_create_model.location,
                    &event_create_model.groupid,
                    &event_create_model.in_person,
                    &event_create_model.time,
                    &event_create_model.duration,
                    &event_create_model.link,
                    &event_create_model.waitlist_count,
                    &event_create_model.is_online,
                    &event_create_model.yes_rsvp_count,
                    &event_create_model.fee,
                    &event_create_model.highres_link,
                    &event_create_model.photo_link,
                    &event_create_model.thumb_link,
                    &event_create_model.rsvp_limit,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn update_by_eventid(
        &self,
        eventid: &i32,
        event_update_model: &EventUpdateModel,
    ) -> Result<EventModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_UPDATE_EVENT_BY_ID).await?;
        let result = &client
        
            .query_one(
                &stmt,
                &[
                    eventid,
                    &event_update_model.name,
                    &event_update_model.description,
                    &event_update_model.location,
                    &event_update_model.groupid,
                    &event_update_model.in_person,
                    &event_update_model.time,
                    &event_update_model.duration,
                    &event_update_model.link,
                    &event_update_model.waitlist_count,
                    &event_update_model.is_online,
                    &event_update_model.yes_rsvp_count,
                    &event_update_model.fee,
                    &event_update_model.highres_link,
                    &event_update_model.photo_link,
                    &event_update_model.thumb_link,
                    &event_update_model.rsvp_limit,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_EVENT_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

impl From<&Row> for EventModel {
    fn from(row: &Row) -> Self {
        Self {
            eventid: row.get("eventid"),
            name: row.get("name"),
            description: row.get("description"),
            extid: row.get("extid"),
            location: row.get("location"),
            groupid: row.get("groupid"),
            in_person: row.get("in_person"),
            time: row.get("time"),
            duration: row.get("duration"),
            link: row.get("link"),
            waitlist_count: row.get("waitlist_count"),
            is_online: row.get("is_online"),
            yes_rsvp_count: row.get("yes_rsvp_count"),
            fee: row.get("fee"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            highres_link: row.get("highres_link"),
            photo_link: row.get("photo_link"),
            thumb_link: row.get("thumb_link"),
            rsvp_limit: row.get("rsvp_limit"),
            
        }
    }
}


impl From<&Row> for EventDetailModel {
    fn from(row: &Row) -> Self {
        Self {
            eventid: row.get("eventid"),
            name: row.get("name"),
            description: row.get("description"),
            location: row.get("location"),
            group_name: row.get("group_name"),
            group_slug: row.get("group_slug"),
            in_person: row.get("in_person"),
            time: row.get("time"),
            duration: row.get("duration"),
            link: row.get("link"),
            waitlist_count: row.get("waitlist_count"),
            is_online: row.get("is_online"),
            yes_rsvp_count: row.get("yes_rsvp_count"),
            fee: row.get("fee"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            highres_link: row.get("highres_link"),
            photo_link: row.get("photo_link"),
            thumb_link: row.get("thumb_link"),
            rsvp_limit: row.get("rsvp_limit"),
            
        }
    }
}
