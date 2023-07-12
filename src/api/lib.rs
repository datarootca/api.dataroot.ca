use actix_web::{
    error::InternalError,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use deadpool_postgres::Pool;
use redis::Client;
use serde_qs::actix::QsQueryConfig;
use std::{error::Error, sync::Arc};

use crate::{
    api::{
        config,
        error::ErrorResponse,
        middleware,
        resources::{health, swagger, categories,state,city,article,group,event},
    },
    domain::{categories::repository::CategoryRepository, health::repository::HealthRepository, state::repository::StateRepository, city::repository::CityRepository, article::repository::ArticleRepository, group::repository::GroupRepository, event::repository::EventRepository},
    repository::{categories::PgCategoryRepository, health::PgHealthRepository,state::PgStateRepository, postgres, city::PgCityRepository, article::PgArticleRepository, event::PgEventRepository, group::PgGroupRepository},
};

pub struct AppState {
    pub health_repository: Arc<dyn HealthRepository>,
    pub category_repository: Arc<dyn CategoryRepository>,
    pub state_repository: Arc<dyn StateRepository>,
    pub city_repository: Arc<dyn CityRepository>,
    pub article_repository: Arc<dyn ArticleRepository>,
    pub group_repository: Arc<dyn GroupRepository>,
    pub event_repository: Arc<dyn EventRepository>,
}

pub async fn run(pg_pool: Arc<Pool>, redis_client: Arc<Client>) -> Result<(), Box<dyn Error>> {
    postgres::run_migrations().await?;

    let json_config = web::JsonConfig::default().error_handler(|err, _| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let query_config = web::QueryConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let path_config = web::PathConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let repositories = Data::new(AppState {
        health_repository: Arc::new(PgHealthRepository::new(
            pg_pool.clone(),
            redis_client.clone(),
        )),
        category_repository: Arc::new(PgCategoryRepository::new(pg_pool.clone())),
        state_repository: Arc::new(PgStateRepository::new(pg_pool.clone())),
        city_repository: Arc::new(PgCityRepository::new(pg_pool.clone())),
        article_repository: Arc::new(PgArticleRepository::new(pg_pool.clone())),
        group_repository: Arc::new(PgGroupRepository::new(pg_pool.clone())),
        event_repository: Arc::new(PgEventRepository::new(pg_pool.clone())),
    });

    let web_addr = &config::get_config().web_addr;
    println!("server listener in: {web_addr}");

    HttpServer::new(move || {
        let qs_config = QsQueryConfig::default()
            .error_handler(|err, _| {
                let http_error =
                    HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
                InternalError::from_response(err, http_error).into()
            })
            .qs_config(serde_qs::Config::new(5, false));

        App::new()
            .wrap(Logger::default())
            .wrap(middleware::cors::default())
            .app_data(json_config.to_owned())
            .app_data(qs_config)
            .app_data(query_config.to_owned())
            .app_data(path_config.to_owned())
            .app_data(repositories.to_owned())
            .configure(swagger::routes::init_routes)
            .configure(health::routes::init_routes)
            .configure(categories::routes::init_routes)
            .configure(city::routes::init_routes)
            .configure(state::routes::init_routes)
            .configure(article::routes::init_routes)
            .configure(group::routes::init_routes)
            .configure(event::routes::init_routes)
    })
    .bind(web_addr)?
    .run()
    .await?;

    Ok(())
}
