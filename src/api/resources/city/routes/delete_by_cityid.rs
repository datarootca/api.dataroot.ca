use actix_web::{
    delete,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    api::lib::AppState,
    domain::{city, error::DomainError},
};

#[utoipa::path(
    delete,
    operation_id = "delete_city",
    path = "/city/{city_id}",
    tag = "city",
    params(
        ("city_id" = Uuid, Path, description = "city uuid"),
    ),
    responses(
         (status = 204, description = "city deleted"),
         (status = 400, description = "Invalid city id",  body = ErrorResponse),
         (status = 404, description = "city not found",  body = ErrorResponse),
         (status = 409, description = "city is in use",  body = ErrorResponse),
    ),
 )]
#[delete("/city/{city_id}")]
async fn handler(
    city: Data<AppState>,
    param: web::Path<Uuid>,
) -> Result<HttpResponse, DomainError> {
    city::resources::delete_by_cityid::execute(
        city.city_repository.clone(),
        param.to_owned(),
    )
    .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use crate::{
        api::{resources::city::routes::init_routes, tests::utils::get_app},
        domain::city::{model::CityCreateModel, repository::CityRepository},
    };

    #[actix_web::test]
    async fn it_should_return_void_city_deleted() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let city_model = CityCreateModel::mock_default();
        repositories
            .city_repository
            .insert(&city_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/city/{}", city_model.cityid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_deleting() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/city/{}", Uuid::new_v4()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
