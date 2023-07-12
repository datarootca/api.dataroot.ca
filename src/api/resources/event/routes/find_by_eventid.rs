use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    api::{
        lib::AppState, resources::event::dto::ResponseEvent, utils::response::ApiResponse,
    },
    domain::{event, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_event_by_eventid",
    path = "/event/{event_id}",
    tag = "event",
    params(
        ("event_id" = Uuid, Path, description = "Event uuid"),
    ),
    responses(
         (status = 200, description = "Event finded",  body = ApiResponseEvent),
         (status = 204, description = "Event no content"),
    ),
 )]
#[get("/event/{event_id}")]
async fn handler(
    event: Data<AppState>,
    param: web::Path<Uuid>,
) -> Result<HttpResponse, DomainError> {
    let result = event::resources::find_by_eventid::execute(
        event.event_repository.clone(),
        param.to_owned(),
    )
    .await?;

    if let Some(event) = result {
        let response =
            ApiResponse::<ResponseEvent>::new(vec![event.into()], None, None, None);

        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use crate::{
        api::{resources::event::routes::init_routes, tests::utils::get_app},
        domain::event::{model::EventCreateModel, repository::EventRepository},
    };

    #[actix_web::test]
    async fn it_should_return_event_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let event_model = EventCreateModel::mock_default();
        repositories
            .event_repository
            .insert(&event_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/event/{}", event_model.eventid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/event/{}", Uuid::new_v4()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }
}
