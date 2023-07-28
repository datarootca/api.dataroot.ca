use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};


use crate::{
    api::{
        lib::AppState, resources::group::dto::ApiResponseDetailGroup,
    },
    domain::{group::{self, model::DetailedGroup},city,state, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_group_by_slug",
    path = "/api/v1/group/{slug}",
    tag = "group",
    params(
        ("slug" = str, Path, description = "Group slug"),
    ),
    responses(
         (status = 200, description = "Group finded",  body = ApiResponseDetailGroup),
         (status = 204, description = "Group no content"),
    ),
 )]
#[get("/api/v1/group/{slug}")]
async fn handler(
    app_state: Data<AppState>,
    param: web::Path<String>,
) -> Result<HttpResponse, DomainError> {
    let result = group::resources::find_by_slug::execute(
        app_state.group_repository.clone(),
        param.to_owned(),
    )
    .await?;

    if let Some(group) = result {
        let city_model = city::resources::find_by_cityid::execute(
            app_state.city_repository.clone(),
            group.cityid
        )
        .await?;
        
        if let Some(city_model) = city_model {
            let state_model = state::resources::find_by_stateid::execute(
                app_state.state_repository.clone(),
                city_model.stateid
            ).await?;
        
            if let Some(state_model) = state_model {
                let detailed_group = DetailedGroup::new(group, city_model, state_model);
                let response: ApiResponseDetailGroup = detailed_group.into();
                return Ok(HttpResponse::Ok().json(response));
            }
        }
    }

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
            .uri(&format!("/api/v1/group/{}", group.slug))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/api/v1/group/{}", random_number().to_string()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }
}
