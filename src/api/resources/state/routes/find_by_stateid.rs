use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};


use crate::{
    api::{
        lib::AppState, resources::state::dto::ResponseState, utils::response::ApiResponse,
    },
    domain::{state, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_state_by_stateid",
    path = "/state/{state_id}",
    tag = "state",
    params(
        ("state_id" = i32, Path, description = "State uuid"),
    ),
    responses(
         (status = 200, description = "State finded",  body = ApiResponseState),
         (status = 204, description = "State no content"),
    ),
 )]
#[get("/state/{state_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<i32>,
) -> Result<HttpResponse, DomainError> {
    let result = state::resources::find_by_stateid::execute(
        state.state_repository.clone(),
        param.to_owned(),
    )
    .await?;

    if let Some(state) = result {
        let response =
            ApiResponse::<ResponseState>::new(vec![state.into()], None, None, None);

        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{resources::state::routes::init_routes, tests::utils::get_app, utils::random_number},
        domain::state::{model::StateCreateModel, repository::StateRepository},
    };

    #[actix_web::test]
    async fn it_should_return_state_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let state_model = StateCreateModel::mock_default();
        let state = repositories
            .state_repository
            .insert(&state_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/state/{}", state.stateid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/state/{}", random_number().to_string()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }
}
