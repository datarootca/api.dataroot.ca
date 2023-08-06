use actix_web::web;

pub mod find;
pub mod find_by_eventid;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_by_eventid::handler);
    config.service(find::handler);
}
