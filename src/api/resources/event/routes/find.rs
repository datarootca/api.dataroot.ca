use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};

use validator::{Validate};

use crate::{
    api::{
        config,
        lib::AppState,
        resources::event::dto::{self, ResponseDetailEvent},
        utils::response::ApiResponse,
    },
    domain::{event, error::DomainError}
};

#[utoipa::path(
    get,
    operation_id = "find_event",
    path = "/event",
    tag = "event",
    params(
        dto::RequestFindEvent
    ),
    responses(
         (status = 200, description = "event",  body = ApiResponseEvent),
         (status = 204, description = "no content event"),
         (status = 400, description = "Invalid query parameters",  body = ErrorResponse),
    ),
 )]
#[get("/event")]
async fn handler(
    state: Data<AppState>,
    query: Query<dto::RequestFindEvent>,
) -> Result<HttpResponse, DomainError> {
    query.validate()?;

    let page = query.page.unwrap_or(1);
    let page_size = query
        .page_size
        .unwrap_or(config::get_config().page_size_default);

    let name = query.name.to_owned();
    let in_person = query.in_person.to_owned();
    let is_online = query.is_online.to_owned();
    let location = query.location.to_owned();
    let group_slug = query.group_slug.to_owned();
    let has_fee = query.has_fee.to_owned();
    let rsvp_limit = query.rsvp_limit.to_owned();
    let status = query.status.to_owned();
    
    let time_frame = query.time_frame.to_owned();
    let start_date = query.start_date.to_owned();
    let end_date = query.end_date.to_owned();

    let result = event::resources::find::execute(
        state.event_repository.clone(),
        name,
        in_person,
        is_online,
        group_slug,
        location,
        has_fee,
        rsvp_limit,
        status,
        time_frame,
        start_date,
        end_date,
        page,
        page_size,
    )
    .await?;

    if let Some((state, count)) = result {
        let response = ApiResponse::<ResponseDetailEvent>::new(
            state.into_iter().map(|i| i.into()).collect(),
            Some(page),
            Some(count),
            Some(page_size),
        );
        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::{
        api::{
            resources::event::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::{event::{model::EventCreateModel, repository::EventRepository}},
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

        let req = test::TestRequest::get().uri("/event").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_event_finded: ApiResponse<dto::ResponseDetailEvent> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_event_finded.records.is_empty());
    }
    #[actix_web::test]
    async fn it_should_return_event_finded_by_query() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let event_model = EventCreateModel::mock_default();
        repositories
            .event_repository
            .insert(&event_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!(
                "/event?name={}&page=1&page_size=24",
                &event_model.name,
            ))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_event_finded: ApiResponse<dto::ResponseDetailEvent> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_event_finded.records.is_empty());
    }

    #[actix_web::test]
    async fn it_should_return_event_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/event?name={}", "no-content",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn it_should_return_bad_request_error_when_query_parameters_is_invalid() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/event?page={}&page_size=24", "invalid",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::BAD_REQUEST);
    }
}
