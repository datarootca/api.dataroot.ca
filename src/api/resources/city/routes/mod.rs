use actix_web::web;

pub mod create;
pub mod delete_by_cityid;
pub mod find;
pub mod find_by_slug;
pub mod find_by_cityid;
pub mod update_by_cityid;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create::handler);
    config.service(update_by_cityid::handler);
    config.service(find_by_cityid::handler);
    config.service(find_by_slug::handler);
    config.service(find::handler);
    config.service(delete_by_cityid::handler);
}
