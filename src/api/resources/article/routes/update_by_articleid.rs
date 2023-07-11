use actix_web::{
    put,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;
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
    put,
    operation_id = "update_article",
    path = "/article/{article_id}",
    tag = "article",
    params(
        ("article_id" = Uuid, Path, description = "Article uuid"),
    ),
    request_body = RequestUpdateArticle,
    responses(
         (status = 200, description = "Article updated",  body = ApiResponseArticle),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
         (status = 404, description = "Article not found",  body = ErrorResponse),
    ),
 )]
#[put("/article/{article_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<Uuid>,
    body: web::Json<dto::RequestUpdateArticle>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let article: article::model::ArticleModel = article::resources::update_by_articleid::execute(
        state.article_repository.clone(),
        param.to_owned(),
        body.0.into(),
    )
    .await?;

    let response = ApiResponse::<ResponseArticle>::new(vec![article.into()], None, None, None);

    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use crate::{
        api::{
            resources::article::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::article::{model::ArticleCreateModel, repository::ArticleRepository},
    };

    #[actix_web::test]
    async fn it_should_return_article_updated() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let article_model = ArticleCreateModel::mock_default();
        repositories
            .article_repository
            .insert(&article_model.clone())
            .await
            .unwrap();

        let mock_request_update_article =
            dto::RequestUpdateArticle::mock_default().with_name("Burgers Supreme");
        let req = test::TestRequest::put()
            .uri(&format!("/article/{}", article_model.articleid))
            .set_json(mock_request_update_article.clone())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let mock_response_article_updated: ApiResponse<dto::ResponseArticle> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_eq!(
            mock_response_article_updated.records.first().unwrap().name,
            mock_request_update_article.name
        )
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_updated_because_invalid_id() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::put()
            .uri(&format!("/article/{}", Uuid::new_v4()))
            .set_json(dto::RequestUpdateArticle::mock_default().with_name("weapons update 3"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
