use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::article::dto::{self, ResponseArticle},
        utils::response::ApiResponse,
    },
    domain::{article, error::DomainError},
};

#[utoipa::path(
    post,
    operation_id = "create_article",
    path = "/article",
    tag = "article",
    request_body = RequestCreateState,
    responses(
         (status = 201, description = "article created",  body = ApiResponseState),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
    ),
 )]
#[post("/article")]
async fn handler(
    state: Data<AppState>,
    body: web::Json<dto::RequestCreateArticle>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let article =
        article::resources::create::execute(state.article_repository.clone(), body.0.into())
            .await?;

    let response = ApiResponse::<ResponseArticle>::new(vec![article.into()], None, None, None);

    Ok(HttpResponse::Created().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::api::{
        resources::article::{dto, routes::init_routes},
        tests::utils::get_app,
    };

    #[actix_web::test]
    async fn it_should_return_article_created() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::post()
            .uri("/article")
            .set_json(dto::RequestCreateArticle::mock_default())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::CREATED);
    }
}
