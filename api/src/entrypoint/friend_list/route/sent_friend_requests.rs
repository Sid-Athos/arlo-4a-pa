use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::friend_list_repository::FriendListRepository;
use crate::domain::model::user::User;
use crate::entrypoint::friend_list::route::request::create_friend_list_request::CreateFriendListRequest;
use crate::entrypoint::friend_list::route::response::friend_list_response::FriendListResponse;
use crate::service::friend_list_service::FriendListService;