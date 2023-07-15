use actix_web::{
    put,
    web::{self, Data},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::city::dto::{self, ResponseCity},
        utils::response::ApiResponse,
    },
    domain::{city, error::DomainError},
};

#[utoipa::path(
    put,
    operation_id = "update_city",
    path = "/city/{city_id}",
    tag = "city",
    params(
        ("city_id" = i32, Path, description = "City uuid"),
    ),
    request_body = RequestUpdateCity,
    responses(
         (status = 200, description = "City updated",  body = ApiResponseCity),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
         (status = 404, description = "City not found",  body = ErrorResponse),
    ),
 )]
#[put("/city/{city_id}")]
async fn handler(
    city: Data<AppState>,
    param: web::Path<i32>,
    body: web::Json<dto::RequestUpdateCity>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let city: city::model::CityModel = city::resources::update_by_cityid::execute(
        city.city_repository.clone(),
        param.to_owned(),
        body.0.into(),
    )
    .await?;

    let response = ApiResponse::<ResponseCity>::new(vec![city.into()], None, None, None);

    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{
            resources::city::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::{response::ApiResponse, random_number},
        },
        domain::city::{model::CityCreateModel, repository::CityRepository},
    };

    #[actix_web::test]
    async fn it_should_return_city_updated() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let city_model = CityCreateModel::mock_default();
        let city = repositories
            .city_repository
            .insert(&city_model.clone())
            .await
            .unwrap();

        let mock_request_update_city =
            dto::RequestUpdateCity::mock_default().with_name("Burgers Supreme");
        let req = test::TestRequest::put()
            .uri(&format!("/city/{}", city.cityid))
            .set_json(mock_request_update_city.clone())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let mock_response_city_updated: ApiResponse<dto::ResponseCity> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_eq!(
            mock_response_city_updated.records.first().unwrap().name,
            mock_request_update_city.name
        )
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_updated_because_invalid_id() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::put()
            .uri(&format!("/city/{}", random_number().to_string()))
            .set_json(dto::RequestUpdateCity::mock_default().with_name("weapons update 3"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
