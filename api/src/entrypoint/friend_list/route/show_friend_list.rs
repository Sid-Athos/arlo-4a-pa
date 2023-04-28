use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::domain::model::user::User;

use crate::entrypoint::friend_list::route::response::friend_list_response::FriendListResponse;
use crate::service::friend_list_service::FriendListService;

#[utoipa::path(
    get,
    path = "/friend_list/",
    responses(
        (status = 200, description = "Friends found", body = Vec<UserResponse>),
        (status = 401, description = "Invalid token",),
    ),
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    ),
    tag="friend_list"
)]
pub async fn show_friend_list(State(pool): State<ConnectionPool>, Extension(user): Extension<User>) -> Result<Json<Vec<FriendListResponse>>, StatusCode> {
    let friend_list_service = FriendListService::new(
        FriendListRepository::new(pool.clone()),
    );

    let result = friend_list_service.get_all_friends(user.id).await?;

    Ok(Json(FriendListResponse::from_vec_domain(result)))
}