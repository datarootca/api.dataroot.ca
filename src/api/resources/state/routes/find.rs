use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        config,
        lib::AppState,
        resources::state::dto::{self, ResponseState},
        utils::response::ApiResponse,
    },
    domain::{state, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_state",
    path = "/state",
    tag = "state",
    params(
        dto::RequestFindCategories
    ),
    responses(
         (status = 200, description = "state",  body = ApiResponseState),
         (status = 204, description = "no content state"),
         (status = 400, description = "Invalid query parameters",  body = ErrorResponse),
    ),
 )]
#[get("/state")]
async fn handler(
    state: Data<AppState>,
    query: Query<dto::RequestFindCategories>,
) -> Result<HttpResponse, DomainError> {
    query.validate()?;

    let page = query.page.unwrap_or(1);
    let page_size = query
        .page_size
        .unwrap_or(config::get_config().page_size_default);

    let name = query.name.to_owned();

    let result = state::resources::find::execute(
        state.state_repository.clone(),
        name,
        page,
        page_size,
    )
    .await?;

    if let Some((state, count)) = result {
        let response = ApiResponse::<ResponseState>::new(
            state.into_iter().map(|i| i.into()).collect(),
            Some(page),
            Some(count),
            Some(page_size),
        );
        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::{
        api::{
            resources::state::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::state::{model::StateCreateModel, repository::StateRepository},
    };

    #[actix_web::test]
    async fn it_should_return_state_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let state_model = StateCreateModel::mock_default();
        repositories
            .state_repository
            .insert(&state_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get().uri("/state").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_state_finded: ApiResponse<dto::ResponseState> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_state_finded.records.is_empty());
    }
    #[actix_web::test]
    async fn it_should_return_state_finded_by_query() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let state_model = StateCreateModel::mock_default();
        repositories
            .state_repository
            .insert(&state_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!(
                "/state?name={}&page=1&page_size=24",
                &state_model.name,
            ))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_state_finded: ApiResponse<dto::ResponseState> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_state_finded.records.is_empty());
    }

    #[actix_web::test]
    async fn it_should_return_bad_request_error_when_query_parameters_is_invalid() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/state?page={}&page_size=24", "invalid",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::BAD_REQUEST);
    }
}
