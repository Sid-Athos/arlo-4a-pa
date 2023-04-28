mod database;
mod service;
mod domain;
mod entrypoint;
mod middlewares;

use std::env;
use axum::Router;
use std::net::SocketAddr;
use axum::http::Method;
use dotenv::dotenv;
use tokio_postgres::Socket;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::middlewares::{tracing::init_tracer, cors_layer::init_cors_layer};
use crate::database::init::init_db;
use crate::entrypoint::admin::admin_router::admin_routes;
use crate::entrypoint::user::user_router::user_routes;
use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::route::request::update_user_request::UpdateUserRequest;
use crate::entrypoint::user::route::request::change_password_request::ChangePasswordRequest;
use crate::entrypoint::user::route::request::login_request::LoginRequest;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::entrypoint::user::route::response::session_response::SessionResponse;

#[tokio::main]
async fn main() {

    dotenv().ok();
    init_tracer();

    let pool = init_db().await.unwrap();

    let cors = init_cors_layer();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/user", user_routes(pool.clone()))
        .nest("/admin", admin_routes(pool.clone()))
        .layer(cors);

    let addr : SocketAddr = (&env::var("SERVER").unwrap()).parse().expect("Not a socket address");

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(
    paths(
        entrypoint::user::route::get_by_id::user_get,
        entrypoint::user::route::create::user_create,
        entrypoint::user::route::login::user_login,
        entrypoint::user::route::logout::user_logout,
        entrypoint::user::route::me::me,
        entrypoint::user::route::search::search,
        entrypoint::user::route::delete::delete_user,
        entrypoint::user::route::update::update_user,
        entrypoint::user::route::change_password::change_password,
        entrypoint::admin::route::get_all::get_all,
        entrypoint::admin::route::delete::delete_user,
        entrypoint::admin::route::give_admin_role::give_admin_role,
        entrypoint::admin::route::remove_admin_role::remove_admin_role,
        entrypoint::admin::route::update_user::update_user,
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
