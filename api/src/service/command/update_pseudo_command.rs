use crate::entrypoint::user::route::request::update_user_request::UpdateUserRequest;

pub struct UpdatePseudoCommand {
    pub id: i32,
    pub pseudo: String
}

impl UpdatePseudoCommand {
    pub fn new(user_id: i32, update_user_request: UpdateUserRequest) -> Self {
        UpdatePseudoCommand {
            id: user_id,
            pseudo: update_user_request.pseudo,
        }
    }
}