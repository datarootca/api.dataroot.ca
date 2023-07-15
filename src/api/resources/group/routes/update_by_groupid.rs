use actix_web::{
    put,
    web::{self, Data},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::group::dto::{self, ResponseGroup},
        utils::response::ApiResponse,
    },
    domain::{group, error::DomainError},
};

#[utoipa::path(
    put,
    operation_id = "update_group",
    path = "/group/{group_id}",
    tag = "group",
    params(
        ("group_id" = i32, Path, description = "Group uuid"),
    ),
    request_body = RequestUpdateGroup,
    responses(
         (status = 200, description = "Group updated",  body = ApiResponseGroup),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
         (status = 404, description = "Group not found",  body = ErrorResponse),
    ),
 )]
#[put("/group/{group_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<i32>,
    body: web::Json<dto::RequestUpdateGroup>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let group: group::model::GroupModel = group::resources::update_by_groupid::execute(
        state.group_repository.clone(),
        param.to_owned(),
        body.0.into(),
    )
    .await?;

    let response = ApiResponse::<ResponseGroup>::new(vec![group.into()], None, None, None);

    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{
            resources::group::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::{response::ApiResponse, random_number},
        },
        domain::group::{model::GroupCreateModel, repository::GroupRepository},
    };

    #[actix_web::test]
    async fn it_should_return_group_updated() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let group_model = GroupCreateModel::mock_default();
        let group = repositories
            .group_repository
            .insert(&group_model.clone())
            .await
            .unwrap();

        let mock_request_update_group =
            dto::RequestUpdateGroup::mock_default().with_name("Burgers Supreme");
        let req = test::TestRequest::put()
            .uri(&format!("/group/{}", group.groupid))
            .set_json(mock_request_update_group.clone())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let mock_response_group_updated: ApiResponse<dto::ResponseGroup> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_eq!(
            mock_response_group_updated.records.first().unwrap().name,
            mock_request_update_group.name
        )
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_updated_because_invalid_id() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::put()
            .uri(&format!("/group/{}", random_number().to_string()))
            .set_json(dto::RequestUpdateGroup::mock_default().with_name("weapons update 3"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
