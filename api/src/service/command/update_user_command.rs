use crate::entrypoint::user::route::request::update_user_request::UpdateUserRequest;

pub struct UpdateUserCommand {
    pub id: i32,
    pub pseudo: String
}

impl UpdateUserCommand {
    pub fn new(user_id: i32, update_user_request: UpdateUserRequest) -> Self {
        UpdateUserCommand {
            id: user_id,
            pseudo: update_user_request.pseudo,
        }
    }
}