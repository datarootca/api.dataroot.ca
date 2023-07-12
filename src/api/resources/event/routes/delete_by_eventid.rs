use actix_web::{
    delete,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    api::lib::AppState,
    domain::{event, error::DomainError},
};

#[utoipa::path(
    delete,
    operation_id = "delete_event",
    path = "/event/{event_id}",
    tag = "event",
    params(
        ("event_id" = Uuid, Path, description = "event uuid"),
    ),
    responses(
         (status = 204, description = "event deleted"),
         (status = 400, description = "Invalid event id",  body = ErrorResponse),
         (status = 404, description = "event not found",  body = ErrorResponse),
         (status = 409, description = "event is in use",  body = ErrorResponse),
    ),
 )]
#[delete("/event/{event_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<Uuid>,
) -> Result<HttpResponse, DomainError> {
    event::resources::delete_by_eventid::execute(
        state.event_repository.clone(),
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
        api::{resources::event::routes::init_routes, tests::utils::get_app},
        domain::event::{model::EventCreateModel, repository::EventRepository},
    };

    #[actix_web::test]
    async fn it_should_return_void_event_deleted() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let event_model = EventCreateModel::mock_default();
        repositories
            .event_repository
            .insert(&event_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/event/{}", event_model.eventid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_deleting() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/event/{}", Uuid::new_v4()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
