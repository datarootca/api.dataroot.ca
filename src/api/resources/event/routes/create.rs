use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::event::dto::{self, ResponseEvent},
        utils::response::ApiResponse,
    },
    domain::{event, error::DomainError},
};

#[utoipa::path(
    post,
    operation_id = "create_event",
    path = "/event",
    tag = "event",
    request_body = RequestCreateState,
    responses(
         (status = 201, description = "event created",  body = ApiResponseState),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
    ),
 )]
#[post("/event")]
async fn handler(
    state: Data<AppState>,
    body: web::Json<dto::RequestCreateEvent>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let event =
        event::resources::create::execute(state.event_repository.clone(), body.0.into())
            .await?;

    let response = ApiResponse::<ResponseEvent>::new(vec![event.into()], None, None, None);

    Ok(HttpResponse::Created().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::api::{
        resources::event::{dto, routes::init_routes},
        tests::utils::get_app,
    };

    #[actix_web::test]
    async fn it_should_return_event_created() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::post()
            .uri("/event")
            .set_json(dto::RequestCreateEvent::mock_default())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::CREATED);
    }
}
