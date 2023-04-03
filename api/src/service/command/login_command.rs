use crate::entrypoint::user::route::request::login_request::LoginRequest;

pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

impl LoginCommand {
    pub fn new(login_request: LoginRequest) -> Self {
        LoginCommand {
            email: login_request.email,
            password: login_request.password,
        }
    }
}