use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::user::User;

use crate::entrypoint::friend_list::route::response::friend_list_response::FriendListResponse;
use crate::entrypoint::friend_list::route::response::user_response::UserResponse;
use crate::service::friend_list_service::FriendListService;
use crate::service::user_service::UserService;

#[utoipa::path(
    delete,
    path = "/friend_list/{user_id}",
    responses(
        (status = 200, description = "FriendList entry deleted", body = FriendListResponse,),
        (status = 401, description = "Invalid token",),
    ),
security(
("api-key" = []),
("bearer" = [])
),
    tag="friend_list"
)]
pub async fn delete_friend(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Path(friend_list_id): Path<i32>) -> Result<Json<FriendListResponse>, StatusCode> {
    let friend_list_service = FriendListService::new(
        FriendListRepository::new(pool.clone()),
    );
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone()),
    );

    let result = friend_list_service.delete_friend(friend_list_id,user.id).await?;

    let recipient = UserResponse::from_domain(user_service.get_user_by_id(result.recipient_id).await?);
    let applicant = UserResponse::from_domain(user_service.get_user_by_id(result.applicant_id).await?);

    Ok(Json(FriendListResponse::from_domain(result, applicant, recipient)))
}