use actix_web::{
    delete,
    web::{self, Data},
    HttpResponse,
};


use crate::{
    api::lib::AppState,
    domain::{article, error::DomainError},
};

#[utoipa::path(
    delete,
    operation_id = "delete_article",
    path = "/article/{article_id}",
    tag = "article",
    params(
        ("article_id" = i32, Path, description = "article uuid"),
    ),
    responses(
         (status = 204, description = "article deleted"),
         (status = 400, description = "Invalid article id",  body = ErrorResponse),
         (status = 404, description = "article not found",  body = ErrorResponse),
         (status = 409, description = "article is in use",  body = ErrorResponse),
    ),
 )]
#[delete("/article/{article_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<i32>,
) -> Result<HttpResponse, DomainError> {
    article::resources::delete_by_articleid::execute(
        state.article_repository.clone(),
        param.to_owned(),
    )
    .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::{
        api::{resources::article::routes::init_routes, tests::utils::get_app, utils::random_number},
        domain::article::{model::ArticleCreateModel, repository::ArticleRepository},
    };

    #[actix_web::test]
    async fn it_should_return_void_article_deleted() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let article_model = ArticleCreateModel::mock_default();
        let article = repositories
            .article_repository
            .insert(&article_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/article/{}", article.articleid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_deleting() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/article/{}", random_number().to_string()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
