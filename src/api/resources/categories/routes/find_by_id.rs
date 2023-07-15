use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};


use crate::{
    api::{
        lib::AppState, resources::categories::dto::ResponseCategory, utils::response::ApiResponse,
    },
    domain::{categories, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_category_by_id",
    path = "/categories/{category_id}",
    tag = "categories",
    params(
        ("category_id" = i32, Path, description = "Category uuid"),
    ),
    responses(
         (status = 200, description = "Category finded",  body = ApiResponseCategory),
         (status = 204, description = "Category no content"),
    ),
 )]
#[get("/categories/{category_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<i32>,
) -> Result<HttpResponse, DomainError> {
    let result = categories::resources::find_by_id::execute(
        state.category_repository.clone(),
        param.to_owned(),
    )
    .await?;

    if let Some(category) = result {
        let response =
            ApiResponse::<ResponseCategory>::new(vec![category.into()], None, None, None);

        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{resources::categories::routes::init_routes, tests::utils::get_app, utils::random_number},
        domain::categories::{model::CategoryCreateModel, repository::CategoryRepository},
    };

    #[actix_web::test]
    async fn it_should_return_category_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let category_model = CategoryCreateModel::mock_default();
        let category = repositories
            .category_repository
            .insert(&category_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/categories/{}", category.id))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/categories/{}", random_number().to_string()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }
}
