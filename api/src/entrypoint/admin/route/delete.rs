use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::entrypoint::admin::route::request::delete_user_request::DeleteUserRequest;
use crate::entrypoint::admin::route::response::user_response::UserResponse;
use crate::service::user_service::UserService;

#[utoipa::path(
    delete,
    path = "/admin/delete",
    request_body = DeleteUserRequest,
    responses(
        (status = 200, description = "User deleted", body = UserResponse),
        (status = 401, description = "Invalid token",),
    ),
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    )
)]
pub async fn delete_user(State(pool): State<ConnectionPool>, Json(delete_user_request): Json<DeleteUserRequest>) -> Result<Json<UserResponse>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.delete_account(delete_user_request.user_id).await?;

    Ok(Json(UserResponse::from_domain(user)))
}