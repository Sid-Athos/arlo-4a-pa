use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::user_response::UserResponse;
use crate::service::friend_list_service::FriendListService;
use crate::service::user_service::UserService;

pub async fn get_connected_friends(State(pool): State<ConnectionPool>, connections: Extension<Connections>, Extension(user): Extension<User>) -> Result<Json<Vec<UserResponse>>, StatusCode> {

    let user_service = UserService::new(pool.clone());
    let friend_list_service = FriendListService::new(pool);

    let list_connected = connections.get_list_connected().await;
    let list_friends = friend_list_service.get_all_friends(user.id).await?;

    let mut result = Vec::new();

    for friend in list_friends {
        if list_connected.contains(&friend.applicant_id) && friend.applicant_id != user.id {
            let user = user_service.get_user_by_id(friend.applicant_id).await?;
            result.push(UserResponse::from_domain(user));
        } else if list_connected.contains(&friend.recipient_id) && friend.recipient_id != user.id {
            let user = user_service.get_user_by_id(friend.recipient_id).await?;
            result.push(UserResponse::from_domain(user));
        }
    }

    Ok(Json(result))
}