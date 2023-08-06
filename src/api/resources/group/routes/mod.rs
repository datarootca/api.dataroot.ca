use actix_web::web;

pub mod find;
pub mod find_by_groupid;
pub mod find_by_slug;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_by_groupid::handler);
    config.service(find_by_slug::handler);
    config.service(find::handler);
}
