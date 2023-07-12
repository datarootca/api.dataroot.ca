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
        resources::event::dto::{self, ResponseEvent},
        utils::response::ApiResponse,
    },
    domain::{event, error::DomainError},
};

#[utoipa::path(
    put,
    operation_id = "update_event",
    path = "/event/{event_id}",
    tag = "event",
    params(
        ("event_id" = Uuid, Path, description = "Event uuid"),
    ),
    request_body = RequestUpdateEvent,
    responses(
         (status = 200, description = "Event updated",  body = ApiResponseEvent),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
         (status = 404, description = "Event not found",  body = ErrorResponse),
    ),
 )]
#[put("/event/{event_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<Uuid>,
    body: web::Json<dto::RequestUpdateEvent>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let event: event::model::EventModel = event::resources::update_by_eventid::execute(
        state.event_repository.clone(),
        param.to_owned(),
        body.0.into(),
    )
    .await?;

    let response = ApiResponse::<ResponseEvent>::new(vec![event.into()], None, None, None);

    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use crate::{
        api::{
            resources::event::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::event::{model::EventCreateModel, repository::EventRepository},
    };

    #[actix_web::test]
    async fn it_should_return_event_updated() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let event_model = EventCreateModel::mock_default();
        repositories
            .event_repository
            .insert(&event_model.clone())
            .await
            .unwrap();

        let mock_request_update_event =
            dto::RequestUpdateEvent::mock_default().with_name("Burgers Supreme");
        let req = test::TestRequest::put()
            .uri(&format!("/event/{}", event_model.eventid))
            .set_json(mock_request_update_event.clone())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let mock_response_event_updated: ApiResponse<dto::ResponseEvent> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_eq!(
            mock_response_event_updated.records.first().unwrap().name,
            mock_request_update_event.name
        )
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_updated_because_invalid_id() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::put()
            .uri(&format!("/event/{}", Uuid::new_v4()))
            .set_json(dto::RequestUpdateEvent::mock_default().with_name("weapons update 3"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
