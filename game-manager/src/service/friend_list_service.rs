use axum::http::StatusCode;
use crate::database::init::ConnectionPool;


use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::domain::error::{database_error_to_status_code};
use crate::domain::model::friend_list::FriendList;

pub struct FriendListService {
    pub friend_list_repository: FriendListRepository
}

impl FriendListService {
    pub fn new(pool: ConnectionPool) -> Self {
        FriendListService {
            friend_list_repository: FriendListRepository::new(pool.clone()),
        }
    }

    pub async fn get_all_friends(&self, user_id : i32) -> Result<Vec<FriendList>, StatusCode> {
        self.friend_list_repository.get_friend_list_by_user( user_id).await.map_err(database_error_to_status_code)
    }
}