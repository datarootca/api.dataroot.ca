use actix_web::{get, http::header, HttpResponse, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::resources::health::routes::check::handler,
        //Category
        crate::api::resources::categories::routes::find_by_id::handler,
        crate::api::resources::categories::routes::find::handler,
        //State
        crate::api::resources::state::routes::find_by_stateid::handler,
        crate::api::resources::state::routes::find::handler,
        //City
        crate::api::resources::city::routes::find_by_cityid::handler,
        crate::api::resources::city::routes::find::handler,
        //Article
        crate::api::resources::article::routes::find_by_articleid::handler,
        crate::api::resources::article::routes::find::handler,
        //Event
        crate::api::resources::event::routes::find_by_eventid::handler,
        crate::api::resources::event::routes::find::handler,
         //Group
         crate::api::resources::group::routes::find_by_groupid::handler,
         crate::api::resources::group::routes::find::handler,
    ),
    components(schemas(
        crate::api::error::ErrorResponse, crate::api::utils::response::Meta,
        //Category
        crate::api::utils::response::ApiResponseCategory,
        crate::api::resources::categories::dto::ResponseCategory,
        //State
        crate::api::utils::response::ApiResponseState,
        crate::api::resources::state::dto::ResponseState,
        //City
        crate::api::utils::response::ApiResponseCity,
        crate::api::resources::city::dto::ResponseCity,
        //Article
        crate::api::utils::response::ApiResponseArticle,
        crate::api::resources::article::dto::ResponseArticle,
        //Group
        crate::api::utils::response::ApiResponseGroup,
        crate::api::resources::group::dto::ResponseGroup,
        //Event
        crate::api::utils::response::ApiResponseEvent,
        crate::api::resources::event::dto::ResponseEvent,
    ))
)]
struct ApiDoc;

#[get("/docs")]
async fn redirect() -> impl Responder {
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/docs/"))
        .finish()
}

pub fn swagger() -> SwaggerUi {
    let mut doc = ApiDoc::openapi();
    doc.info.title = String::from("api.dataroot.ca docs");
    doc.info.description = Some(String::from("docs for api.dataroot.ca."));

    SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", doc)
}

#[cfg(test)]
mod tests {

    use crate::api::{middleware, resources::swagger::routes::init_routes};
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_swagger() {
        dotenv::from_filename(".env.test").ok();

        let app = test::init_service(
            App::new()
                .wrap(middleware::cors::default())
                .configure(init_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/docs/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get().uri("/docs").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }
}
