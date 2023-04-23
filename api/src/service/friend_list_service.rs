use std::ptr::null;
use bcrypt::{hash, verify};
use axum::http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use crate::database::database_error::{database_error_invalid_input, DatabaseError};
use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::domain::error::{database_error_to_status_code, internal_error};
use crate::domain::model::friend_list::FriendList;

pub struct FriendListService {
    pub friend_list_repository: FriendListRepository
}

impl FriendListService {
    pub fn new(friend_list_repository: FriendListRepository) -> Self {
        FriendListService {
            friend_list_repository
        }
    }

    pub async fn create_friend_request(&self, recipient_id: i32, applicant_id: i32) -> Result<FriendList, StatusCode> {
        let exist = self.friend_list_repository.get_friend_list_request_by_users(applicant_id, recipient_id).await.map_err(database_error_to_status_code);

        return if exist.is_ok() {
            self.friend_list_repository.accept_friend_list_request(exist.unwrap().id,applicant_id).await.map_err(database_error_to_status_code)
        } else {
            self.friend_list_repository.create_friend_list_request(recipient_id, applicant_id).await.map_err(database_error_to_status_code)
        }
    }

    pub async fn delete_friend(&self, request_id: i32, user_id: i32) -> Result<FriendList, StatusCode> {
        self.friend_list_repository.delete_friend_list_request(request_id, user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn accept_friend_request(&self, request_id : i32, user_id : i32) -> Result<FriendList, StatusCode> {
        self.friend_list_repository.accept_friend_list_request(request_id, user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_all_friends(&self, user_id : i32) -> Result<Vec<FriendList>, StatusCode> {
        self.friend_list_repository.get_friend_list_by_user( user_id).await.map_err(database_error_to_status_code)
    }
}