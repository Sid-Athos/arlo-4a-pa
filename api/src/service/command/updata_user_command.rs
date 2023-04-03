use crate::entrypoint::admin::route::request::update_user_request::UpdateUserRequest;

pub struct UpdateUserCommand {
    pub id: i32,
    pub pseudo: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserCommand {

    pub fn new(user_id: i32, update_user_request: UpdateUserRequest) -> Self {
        UpdateUserCommand {
            id: user_id,
            pseudo: update_user_request.pseudo,
            email: update_user_request.email,
            password: update_user_request.password,
        }
    }
}