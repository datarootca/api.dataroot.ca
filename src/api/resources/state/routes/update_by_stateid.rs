use actix_web::{
    put,
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
    put,
    operation_id = "update_state",
    path = "/state/{state_id}",
    tag = "state",
    params(
        ("state_id" = i32, Path, description = "State uuid"),
    ),
    request_body = RequestUpdateState,
    responses(
         (status = 200, description = "State updated",  body = ApiResponseState),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
         (status = 404, description = "State not found",  body = ErrorResponse),
    ),
 )]
#[put("/state/{state_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<i32>,
    body: web::Json<dto::RequestUpdateState>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let state = state::resources::update_by_stateid::execute(
        state.state_repository.clone(),
        param.to_owned(),
        body.0.into(),
    )
    .await?;

    let response = ApiResponse::<ResponseState>::new(vec![state.into()], None, None, None);

    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{
            resources::state::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::{response::ApiResponse, random_number},
        },
        domain::state::{model::StateCreateModel, repository::StateRepository},
    };

    #[actix_web::test]
    async fn it_should_return_state_updated() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let state_model = StateCreateModel::mock_default();
        let state = repositories
            .state_repository
            .insert(&state_model.clone())
            .await
            .unwrap();

        let mock_request_update_state =
            dto::RequestUpdateState::mock_default().with_name("Slovakia");
        let req = test::TestRequest::put()
            .uri(&format!("/state/{}", state.stateid))
            .set_json(mock_request_update_state.clone())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let mock_response_state_updated: ApiResponse<dto::ResponseState> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_eq!(
            mock_response_state_updated.records.first().unwrap().name,
            mock_request_update_state.name
        )
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_updated_because_invalid_id() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::put()
            .uri(&format!("/state/{}", random_number().to_string()))
            .set_json(dto::RequestUpdateState::mock_default().with_name("weapons update 3"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
