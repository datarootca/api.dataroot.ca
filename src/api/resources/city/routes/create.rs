use actix_web::{
    post,
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
    post,
    operation_id = "create_city",
    path = "/city",
    tag = "city",
    request_body = RequestCreateState,
    responses(
         (status = 201, description = "city created",  body = ApiResponseState),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
    ),
 )]
#[post("/city")]
async fn handler(
    city: Data<AppState>,
    body: web::Json<dto::RequestCreateCity>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let city =
        city::resources::create::execute(city.city_repository.clone(), body.0.into())
            .await?;

    let response = ApiResponse::<ResponseCity>::new(vec![city.into()], None, None, None);

    Ok(HttpResponse::Created().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::api::{
        resources::city::{dto, routes::init_routes},
        tests::utils::get_app,
    };

    #[actix_web::test]
    async fn it_should_return_city_created() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::post()
            .uri("/city")
            .set_json(dto::RequestCreateCity::mock_default())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::CREATED);
    }
}
