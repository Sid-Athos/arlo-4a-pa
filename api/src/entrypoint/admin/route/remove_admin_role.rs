use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::admin::route::response::user_response::UserResponse;
use crate::service::user_service::UserService;

#[utoipa::path(
    put,
    path = "/admin/remove_admin_role/{user_id}",
    params(
        ("user_id" = String,),
    ),
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 401, description = "Invalid token",),
        (status = 404, description = "User not found",),
    ),
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    )
)]
pub async fn remove_admin_role(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>) -> Result<Json<UserResponse>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.remove_admin_role(user_id).await?;

    Ok(Json(UserResponse::from_domain(user)))
}