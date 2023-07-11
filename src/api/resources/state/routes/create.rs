use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::state::dto::{self, ResponseState},
        utils::response::ApiResponse,
    },
    domain::{state, error::DomainError},
};

#[utoipa::path(
    post,
    operation_id = "create_state",
    path = "/state",
    tag = "state",
    request_body = RequestCreateState,
    responses(
         (status = 201, description = "state created",  body = ApiResponseState),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
    ),
 )]
#[post("/state")]
async fn handler(
    state: Data<AppState>,
    body: web::Json<dto::RequestCreateState>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let state =
        state::resources::create::execute(state.state_repository.clone(), body.0.into())
            .await?;

    let response = ApiResponse::<ResponseState>::new(vec![state.into()], None, None, None);

    Ok(HttpResponse::Created().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::api::{
        resources::state::{dto, routes::init_routes},
        tests::utils::get_app,
    };

    #[actix_web::test]
    async fn it_should_return_state_created() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::post()
            .uri("/state")
            .set_json(dto::RequestCreateState::mock_default())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::CREATED);
    }
}
