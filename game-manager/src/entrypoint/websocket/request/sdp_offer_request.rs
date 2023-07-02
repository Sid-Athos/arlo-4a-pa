use axum::Extension;
use axum::http::StatusCode;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::entrypoint::websocket::response::sdp_offer_response::SDPOfferResponse;
use crate::entrypoint::websocket::response::user_response::UserResponse;
use crate::service::lobby_service::LobbyService;
use crate::service::ws_session_service::WsSessionService;

#[derive(Deserialize, Debug)]
pub struct SDPOfferRequest {
    sdp: String,
    to_user_id: i32,
}

impl SDPOfferRequest {
    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let ws_session_service = WsSessionService::new(pool.clone());

        let to_user_session = ws_session_service.get_by_user_id(self.to_user_id).await.map_err(status_code_to_string)?;
        let from_user_session = ws_session_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;
        if to_user_session.lobby_id != from_user_session.lobby_id {
            return Err(StatusCode::UNAUTHORIZED.to_string());
        }

        let response = SDPOfferResponse::new(self.sdp.clone(), UserResponse::from_domain(user));

        connections.send_to_vec_user_id(ResponseEnum::SDPOffer(response), vec![self.to_user_id]).await;

        Ok(())
    }
}