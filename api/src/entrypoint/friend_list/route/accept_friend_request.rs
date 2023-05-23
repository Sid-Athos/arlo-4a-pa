use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::domain::model::user::User;

use crate::entrypoint::friend_list::route::response::friend_list_response::FriendListResponse;
use crate::service::friend_list_service::FriendListService;

#[utoipa::path(
    put,
    path = "/friend_list/{friend_list_id}",
    responses(
        (status = 200, description = "FriendList entry accepted", body = FriendListResponse,),
        (status = 401, description = "Invalid token",),
    ),
    security(
        ("api-key" = [])
    ),
    tag="friend_list"
)]
pub async fn accept_friend_request(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Path(friend_list_id): Path<i32>) -> Result<Json<FriendListResponse>, StatusCode> {
    let friend_list_service = FriendListService::new(
        FriendListRepository::new(pool.clone()),
    );

    let result = friend_list_service.accept_friend_request(friend_list_id,user.id).await?;

    Ok(Json(FriendListResponse::from_domain(result)))
}