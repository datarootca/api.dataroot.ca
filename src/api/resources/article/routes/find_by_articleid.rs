use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    api::{
        lib::AppState, resources::article::dto::ResponseArticle, utils::response::ApiResponse,
    },
    domain::{article, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_article_by_articleid",
    path = "/article/{article_id}",
    tag = "article",
    params(
        ("article_id" = Uuid, Path, description = "Article uuid"),
    ),
    responses(
         (status = 200, description = "Article finded",  body = ApiResponseArticle),
         (status = 204, description = "Article no content"),
    ),
 )]
#[get("/article/{article_id}")]
async fn handler(
    article: Data<AppState>,
    param: web::Path<Uuid>,
) -> Result<HttpResponse, DomainError> {
    let result = article::resources::find_by_articleid::execute(
        article.article_repository.clone(),
        param.to_owned(),
    )
    .await?;

    if let Some(article) = result {
        let response =
            ApiResponse::<ResponseArticle>::new(vec![article.into()], None, None, None);

        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use crate::{
        api::{resources::article::routes::init_routes, tests::utils::get_app},
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

        let req = test::TestRequest::get()
            .uri(&format!("/article/{}", article_model.articleid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/article/{}", Uuid::new_v4()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }
}
