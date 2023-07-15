use actix_web::{
    delete,
    web::{self, Data},
    HttpResponse,
};


use crate::{
    api::lib::AppState,
    domain::{group, error::DomainError},
};

#[utoipa::path(
    delete,
    operation_id = "delete_group",
    path = "/group/{group_id}",
    tag = "group",
    params(
        ("group_id" = i32, Path, description = "group uuid"),
    ),
    responses(
         (status = 204, description = "group deleted"),
         (status = 400, description = "Invalid group id",  body = ErrorResponse),
         (status = 404, description = "group not found",  body = ErrorResponse),
         (status = 409, description = "group is in use",  body = ErrorResponse),
    ),
 )]
#[delete("/group/{group_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<i32>,
) -> Result<HttpResponse, DomainError> {
    group::resources::delete_by_groupid::execute(
        state.group_repository.clone(),
        param.to_owned(),
    )
    .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{resources::group::routes::init_routes, tests::utils::get_app, utils::random_number},
        domain::group::{model::GroupCreateModel, repository::GroupRepository},
    };

    #[actix_web::test]
    async fn it_should_return_void_group_deleted() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let group_model = GroupCreateModel::mock_default();
        let group = repositories
            .group_repository
            .insert(&group_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/group/{}", group.groupid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_deleting() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/group/{}", random_number().to_string()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
