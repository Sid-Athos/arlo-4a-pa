use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::entrypoint::user::route::request::update_user_request::UpdateUserRequest;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::command::updata_user_command::UpdateUserCommand;
use crate::service::user_service::UserService;

#[utoipa::path(
    put,
    path = "/user/update",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 401, description = "Invalid token",),
        (status = 409, description = "Pseudo already used",),
    ),
    request_body = UpdateUserRequest,
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    )
)]
pub async fn update_user(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Json(update_request): Json<UpdateUserRequest>) -> Result<Json<UserResponse>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let command = UpdateUserCommand::new(user.id, Some(update_request.pseudo), None, None);

    let user = user_service.update_user(command).await?;

    Ok(Json(UserResponse::from_domain(user)))
}