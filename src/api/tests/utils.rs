use std::sync::Arc;

use crate::{
    api::{error::ErrorResponse, lib::AppState, middleware},
    repository::{
        categories::PgCategoryRepository,
        state::PgStateRepository,
        health::PgHealthRepository,
        postgres::{self, init_to_tests},
        redis, city::PgCityRepository, 
        article::PgArticleRepository, 
        group::PgGroupRepository, 
        event::PgEventRepository,
    },
};

use tokio::sync::OnceCell;

use actix_http::Request;

use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    error::InternalError,
    test,
    web::{Data, ServiceConfig},
    App, Error, HttpResponse,
};

static INIT_DB: OnceCell<()> = OnceCell::const_new();

async fn setup() {
    INIT_DB
        .get_or_init(|| async {
            dotenv::from_filename(".env.test").ok();

            init_to_tests()
                .await
                .expect("Error to init database to tests");
        })
        .await;
}

pub struct Repositories {
    pub health_repository: Arc<PgHealthRepository>,
    pub category_repository: Arc<PgCategoryRepository>,
    pub state_repository: Arc<PgStateRepository>,
    pub city_repository: Arc<PgCityRepository>,
    pub article_repository: Arc<PgArticleRepository>,
    pub group_repository: Arc<PgGroupRepository>,
    pub event_repository: Arc<PgEventRepository>,
}

impl Repositories {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        health_repository: Arc<PgHealthRepository>,
        category_repository: Arc<PgCategoryRepository>,
        state_repository: Arc<PgStateRepository>,
        city_repository: Arc<PgCityRepository>,
        article_repository: Arc<PgArticleRepository>,
        group_repository: Arc<PgGroupRepository>,
        event_repository: Arc<PgEventRepository>,
    ) -> Self {
        Self {
            health_repository,
            category_repository,
            state_repository,
            city_repository,
            article_repository,
            group_repository,
            event_repository,
        }
    }
}

impl AppState {
    fn mock_default(repositories: &Repositories) -> Data<Self> {
        Data::new(Self {
            health_repository: repositories.health_repository.clone(),
            category_repository: repositories.category_repository.clone(),
            state_repository: repositories.state_repository.clone(),
            city_repository: repositories.city_repository.clone(),
            article_repository: repositories.article_repository.clone(),
            group_repository: repositories.group_repository.clone(),
            event_repository: repositories.event_repository.clone(),
        })
    }
}

pub async fn get_app<F>(
    routes: F,
) -> (
    Repositories,
    impl Service<Request, Response = ServiceResponse<impl MessageBody>, Error = Error>,
)
where
    F: FnOnce(&mut ServiceConfig),
{
    setup().await;

    let json_config = actix_web::web::JsonConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let query_config = actix_web::web::QueryConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let path_config = actix_web::web::PathConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let pool = Arc::new(postgres::init().unwrap());
    let redis_client = Arc::new(redis::init());

    let health_repository = Arc::new(PgHealthRepository::new(pool.clone(), redis_client.clone()));
    let category_repository = Arc::new(PgCategoryRepository::new(pool.clone()));
    let state_repository = Arc::new(PgStateRepository::new(pool.clone()));
    let city_repository = Arc::new(PgCityRepository::new(pool.clone(),redis_client.clone()));
    let article_repository = Arc::new(PgArticleRepository::new(pool.clone()));
    let group_repository = Arc::new(PgGroupRepository::new(pool.clone()));
    let event_repository = Arc::new(PgEventRepository::new(pool.clone()));

    let repositories = Repositories::new(
        health_repository, 
        category_repository,
        state_repository,
        city_repository,
        article_repository,
        group_repository,
        event_repository,
    );

    let app_state = AppState::mock_default(&repositories);

    (
        repositories,
        test::init_service(
            App::new()
                .wrap(middleware::cors::default())
                .app_data(json_config.to_owned())
                .app_data(query_config.to_owned())
                .app_data(path_config.to_owned())
                .app_data(app_state)
                .configure(routes),
        )
        .await,
    )
}
