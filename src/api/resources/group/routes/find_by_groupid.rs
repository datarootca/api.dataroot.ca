use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};


use crate::{
    api::{
        lib::AppState, resources::group::dto::ResponseGroup, utils::response::ApiResponse,
    },
    domain::{group, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_group_by_groupid",
    path = "/group/{group_id}",
    tag = "group",
    params(
        ("group_id" = i32, Path, description = "Group uuid"),
    ),
    responses(
         (status = 200, description = "Group finded",  body = ApiResponseGroup),
         (status = 204, description = "Group no content"),
    ),
 )]
#[get("/group/{group_id}")]
async fn handler(
    group: Data<AppState>,
    param: web::Path<i32>,
) -> Result<HttpResponse, DomainError> {
    let result = group::resources::find_by_groupid::execute(
        group.group_repository.clone(),
        param.to_owned(),
    )
    .await?;

    if let Some(group) = result {
        let response =
            ApiResponse::<ResponseGroup>::new(vec![group.into()], None, None, None);

        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    

    use crate::{
        api::{resources::group::routes::init_routes, tests::utils::get_app, utils::random_number},
        domain::group::{ repository::GroupRepository},
    };

    #[actix_web::test]
    async fn it_should_return_group_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let group_model = GroupCreateModel::mock_default();
        let group = repositories
            .group_repository
            .insert(&group_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/group/{}", group.groupid))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/group/{}", random_number().to_string()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }
}
