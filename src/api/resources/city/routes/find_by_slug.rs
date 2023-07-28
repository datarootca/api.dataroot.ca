use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};


use crate::{
    api::{
        lib::AppState, resources::city::dto::{ResponseDetailCity}, utils::response::ApiResponse,
    },
    domain::{city, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_city_by_slug",
    path = "/api/v1/city/{slug}",
    tag = "city",
    params(
        ("slug" = String, Path, description = "City slug"),
    ),
    responses(
         (status = 200, description = "City finded",  body = ApiResponseCity),
         (status = 204, description = "City no content"),
    ),
 )]
#[get("/api/v1/city/{slug}")]
async fn handler(
    city: Data<AppState>,
    param: web::Path<String>,
) -> Result<HttpResponse, DomainError> {
    let result = city::resources::find_by_slug::execute(
        city.city_repository.clone(),
        param.to_owned(),
    )
    .await?;

    if let Some(city) = result {
        let response =
            ApiResponse::<ResponseDetailCity>::new(vec![city.into()], None, None, None);

        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{resources::city::routes::init_routes, tests::utils::get_app, utils::random_number},
        domain::city::{model::CityCreateModel, repository::CityRepository},
    };

    #[actix_web::test]
    async fn it_should_return_city_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let city_model = CityCreateModel::mock_default();
        let city = repositories
            .city_repository
            .insert(&city_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/api/v1/city/{}", city.slug))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/api/v1/city/{}", random_number().to_string()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }
}
