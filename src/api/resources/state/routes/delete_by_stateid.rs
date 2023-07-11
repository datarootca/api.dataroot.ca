use actix_web::{
    delete,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    api::lib::AppState,
    domain::{state, error::DomainError},
};

#[utoipa::path(
    delete,
    operation_id = "delete_state",
    path = "/state/{state_id}",
    tag = "state",
    params(
        ("state_id" = Uuid, Path, description = "state uuid"),
    ),
    responses(
         (status = 204, description = "state deleted"),
         (status = 400, description = "Invalid state id",  body = ErrorResponse),
         (status = 404, description = "state not found",  body = ErrorResponse),
         (status = 409, description = "state is in use",  body = ErrorResponse),
    ),
 )]
#[delete("/state/{state_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<Uuid>,
) -> Result<HttpResponse, DomainError> {
    state::resources::delete_by_stateid::execute(
        state.state_repository.clone(),
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
        api::{resources::state::routes::init_routes, tests::utils::get_app},
        domain::state::{model::StateCreateModel, repository::StateRepository},
    };

    #[actix_web::test]
    async fn it_should_return_void_state_deleted() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let state_model = StateCreateModel::mock_default();
        repositories
            .state_repository
            .insert(&state_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/state/{}", state_model.stateid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_deleting() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/state/{}", Uuid::new_v4()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
