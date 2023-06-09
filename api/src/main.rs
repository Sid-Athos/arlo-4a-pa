mod database;
mod service;
mod domain;
mod entrypoint;
mod middlewares;

use std::env;
use axum::{ Router};
use std::net::SocketAddr;
use axum::http::{HeaderMap, StatusCode};
use dotenv::dotenv;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

use crate::middlewares::{tracing::init_tracer, cors_layer::init_cors_layer};
use crate::database::init::init_db;


use crate::entrypoint::ranking::ranking_router::ranking_routes;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::entrypoint::user::route::response::session_response::SessionResponse;


use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;
use crate::entrypoint::admin::admin_router::admin_routes;
use crate::entrypoint::friend_list::friend_list_router::friend_list_routes;
use crate::entrypoint::user::user_router::user_routes;
use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::route::request::update_user_request::UpdateUserRequest;
use crate::entrypoint::user::route::request::change_password_request::ChangePasswordRequest;
use crate::entrypoint::user::route::request::login_request::LoginRequest;

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
        .nest("/ranking", ranking_routes(pool.clone()))
        .nest("/friend_list", friend_list_routes(pool.clone()))
        .layer(cors);

    let addr : SocketAddr = (&env::var("SERVER").unwrap()).parse().expect("Not a socket address");

    println!("{} : listening on {}", "START", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}


#[derive(OpenApi)]
#[openapi(
paths(
entrypoint::user::route::get_by_id::user_get,
entrypoint::user::route::create::user_create,
entrypoint::user::route::login::user_login,
entrypoint::user::route::logout::user_logout,
entrypoint::user::route::add_experience::add_experience,
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
entrypoint::friend_list::route::create::friend_list_create,
entrypoint::friend_list::route::delete_friend::delete_friend,
entrypoint::friend_list::route::accept_friend_request::accept_friend_request,
entrypoint::friend_list::route::show_friend_list::show_friend_list,
entrypoint::friend_list::route::show_friend_request::show_friend_request,
entrypoint::ranking::route::init_ranking::init_ranking,
entrypoint::ranking::route::update_ranking::update_ranking,
entrypoint::ranking::route::get_ranking_by_friend::get_ranking_by_friend,
entrypoint::ranking::route::get_ranking_by_game::get_ranking_by_game_id,
entrypoint::ranking::route::get_ranking_by_user::get_ranking_by_user_id,
),
modifiers(&SecurityAddon),
components(
schemas(UserResponse),
schemas(SessionResponse),
schemas(LoginRequest),
schemas(CreateUserRequest),
schemas(ChangePasswordRequest),
schemas(UpdateUserRequest),

),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("api-key"))),
            );
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build())
            )
        }
    }
}
fn check_api_key(
    headers: &HeaderMap
) -> Result<(), (StatusCode, String)> {
    println!("Headers {:?}", headers);
    match headers.get("api-key") {
        Some(header) if header != &env::var("API_KEY").unwrap() => {
            tracing::error_span!("Invalid api key");
            Err((
                StatusCode::UNAUTHORIZED,
                "Incorrect or missing Api Key".to_string(),
            ))
        },
        None => {
            tracing::error_span!("Missing api key");
            Err((
                StatusCode::UNAUTHORIZED,
                "Incorrect or missing Api Key".to_string(),
            ))
        },
        _ => {
            tracing::info!("Valid api-key provided");
            Ok(())
        },
    }
}
