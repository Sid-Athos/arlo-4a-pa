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
use crate::entrypoint::user::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::request::update_user_request::UpdateUserRequest;
use crate::entrypoint::user::request::change_password_request::ChangePasswordRequest;
use crate::entrypoint::user::request::login_request::LoginRequest;
use crate::entrypoint::user::response::user_response::UserResponse;
use crate::entrypoint::user::response::session_response::SessionResponse;

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
        entrypoint::user::user_entry_point::delete_user,
        entrypoint::user::user_entry_point::update_user,
    ),
    components(
        schemas(UserResponse),
        schemas(SessionResponse),
        schemas(LoginRequest),
        schemas(CreateUserRequest),
        schemas(ChangePasswordRequest),
        schemas(UpdateUserRequest),
    )
)]
struct ApiDoc;
