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
        resources::group::dto::{self, ResponsePageGroup},
        utils::response::ApiResponse,
    },
    domain::{group, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_group",
    path = "/group",
    tag = "group",
    params(
        dto::RequestFindGroup
    ),
    responses(
         (status = 200, description = "group",  body = ApiResponseGroup),
         (status = 204, description = "no content group"),
         (status = 400, description = "Invalid query parameters",  body = ErrorResponse),
    ),
 )]
#[get("/group")]
async fn handler(
    state: Data<AppState>,
    query: Query<dto::RequestFindGroup>,
) -> Result<HttpResponse, DomainError> {
    query.validate()?;

    let page = query.page.unwrap_or(1);
    let page_size = query
        .page_size
        .unwrap_or(config::get_config().page_size_default);

    let name = query.name.to_owned();
    let city = query.city.to_owned();

    let result = group::resources::find::execute(
        state.group_repository.clone(),
        name,
        city,
        page,
        page_size,
    )
    .await?;

    if let Some((state, count)) = result {
        let response = ApiResponse::<ResponsePageGroup>::new(
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
            resources::group::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::group::{model::GroupCreateModel, repository::GroupRepository},
    };

    #[actix_web::test]
    async fn it_should_return_group_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let group_model = GroupCreateModel::mock_default();
        repositories
            .group_repository
            .insert(&group_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get().uri("/group").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_group_finded: ApiResponse<dto::ResponsePageGroup> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_group_finded.records.is_empty());
    }
    #[actix_web::test]
    async fn it_should_return_group_finded_by_query() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let group_model = GroupCreateModel::mock_default();
        repositories
            .group_repository
            .insert(&group_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!(
                "/group?name={}&page=1&page_size=24",
                &group_model.name,
            ))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_group_finded: ApiResponse<dto::ResponsePageGroup> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_group_finded.records.is_empty());
    }

    #[actix_web::test]
    async fn it_should_return_group_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/group?name={}", "no-content",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn it_should_return_bad_request_error_when_query_parameters_is_invalid() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/group?page={}&page_size=24", "invalid",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::BAD_REQUEST);
    }
}
