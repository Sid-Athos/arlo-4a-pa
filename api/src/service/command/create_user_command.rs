use crate::entrypoint::user::request::create_user_request::CreateUserRequest;

pub struct CreateUserCommand {
    pub pseudo: String,
    pub email: String,
    pub password: String,
}

impl CreateUserCommand {

    pub fn new(request: CreateUserRequest) -> Self {
        CreateUserCommand {
            pseudo: request.pseudo,
            email: request.email,
            password: request.password,
        }
    }
}