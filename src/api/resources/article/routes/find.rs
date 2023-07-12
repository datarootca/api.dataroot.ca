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
        resources::article::dto::{self, ResponseArticle},
        utils::response::ApiResponse,
    },
    domain::{article, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_article",
    path = "/article",
    tag = "article",
    params(
        dto::RequestFindArticle
    ),
    responses(
         (status = 200, description = "article",  body = ApiResponseArticle),
         (status = 204, description = "no content article"),
         (status = 400, description = "Invalid query parameters",  body = ErrorResponse),
    ),
 )]
#[get("/article")]
async fn handler(
    state: Data<AppState>,
    query: Query<dto::RequestFindArticle>,
) -> Result<HttpResponse, DomainError> {
    query.validate()?;

    let page = query.page.unwrap_or(1);
    let page_size = query
        .page_size
        .unwrap_or(config::get_config().page_size_default);

    let name = query.name.to_owned();

    let result = article::resources::find::execute(
        state.article_repository.clone(),
        name,
        page,
        page_size,
    )
    .await?;

    if let Some((state, count)) = result {
        let response = ApiResponse::<ResponseArticle>::new(
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
            resources::article::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::article::{model::ArticleCreateModel, repository::ArticleRepository},
    };

    #[actix_web::test]
    async fn it_should_return_article_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let article_model = ArticleCreateModel::mock_default();
        repositories
            .article_repository
            .insert(&article_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get().uri("/article").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_article_finded: ApiResponse<dto::ResponseArticle> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_article_finded.records.is_empty());
    }
    #[actix_web::test]
    async fn it_should_return_article_finded_by_query() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let state_model = ArticleCreateModel::mock_default();
        repositories
            .article_repository
            .insert(&state_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!(
                "/article?name={}&page=1&page_size=24",
                &state_model.name,
            ))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_article_finded: ApiResponse<dto::ResponseArticle> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_article_finded.records.is_empty());
    }

    #[actix_web::test]
    async fn it_should_return_article_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/article?name={}", "no-content",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn it_should_return_bad_request_error_when_query_parameters_is_invalid() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/article?page={}&page_size=24", "invalid",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::BAD_REQUEST);
    }
}
