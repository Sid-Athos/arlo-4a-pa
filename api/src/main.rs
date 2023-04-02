mod database;
mod service;
mod domain;
mod entrypoint;

use axum::Router;
use std::net::SocketAddr;
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::database::init::init_db;
use crate::entrypoint::user::user_entry_point::get_routes;
use crate::domain::model::user::User;
use crate::domain::model::session::Session;
use crate::entrypoint::user::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::request::login_request::LoginRequest;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let pool = init_db().await.unwrap();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/user", get_routes(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(
    paths(
        entrypoint::user::user_entry_point::user_get,
        entrypoint::user::user_entry_point::user_create,
        entrypoint::user::user_entry_point::user_login,
        entrypoint::user::user_entry_point::user_logout,
        entrypoint::user::user_entry_point::me,
        entrypoint::user::user_entry_point::search,
    ),
    components(
        schemas(User),
        schemas(Session),
        schemas(LoginRequest),
        schemas(CreateUserRequest),
    )
)]
struct ApiDoc;
