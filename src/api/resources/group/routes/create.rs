use actix_web::{
    post,
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
    post,
    operation_id = "create_group",
    path = "/group",
    tag = "group",
    request_body = RequestCreateState,
    responses(
         (status = 201, description = "group created",  body = ApiResponseState),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
    ),
 )]
#[post("/group")]
async fn handler(
    state: Data<AppState>,
    body: web::Json<dto::RequestCreateGroup>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let group =
        group::resources::create::execute(state.group_repository.clone(), body.0.into())
            .await?;

    let response = ApiResponse::<ResponseGroup>::new(vec![group.into()], None, None, None);

    Ok(HttpResponse::Created().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::api::{
        resources::group::{dto, routes::init_routes},
        tests::utils::get_app,
    };

    #[actix_web::test]
    async fn it_should_return_group_created() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::post()
            .uri("/group")
            .set_json(dto::RequestCreateGroup::mock_default())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::CREATED);
    }
}
