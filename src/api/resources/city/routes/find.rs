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
        resources::city::dto::{self, ResponseCity},
        utils::response::ApiResponse,
    },
    domain::{city, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_city",
    path = "/city",
    tag = "city",
    params(
        dto::RequestFindCategories
    ),
    responses(
         (status = 200, description = "city",  body = ApiResponseCity),
         (status = 204, description = "no content city"),
         (status = 400, description = "Invalid query parameters",  body = ErrorResponse),
    ),
 )]
#[get("/city")]
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

    let result = city::resources::find::execute(
        state.city_repository.clone(),
        name,
        page,
        page_size,
    )
    .await?;

    if let Some((state, count)) = result {
        let response = ApiResponse::<ResponseCity>::new(
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
            resources::city::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::city::{model::CityCreateModel, repository::CityRepository},
    };

    #[actix_web::test]
    async fn it_should_return_city_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let city_model = CityCreateModel::mock_default();
        repositories
            .city_repository
            .insert(&city_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get().uri("/city").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_city_finded: ApiResponse<dto::ResponseCity> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_city_finded.records.is_empty());
    }
    #[actix_web::test]
    async fn it_should_return_city_finded_by_query() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let city_model = CityCreateModel::mock_default();
        repositories
            .city_repository
            .insert(&city_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!(
                "/city?name={}&page=1&page_size=24",
                city_model.name,
            ))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_city_finded: ApiResponse<dto::ResponseCity> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_city_finded.records.is_empty());
    }

    #[actix_web::test]
    async fn it_should_return_city_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/city?name={}", "no-content",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn it_should_return_bad_request_error_when_query_parameters_is_invalid() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/city?page={}&page_size=24", "invalid",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::BAD_REQUEST);
    }
}
