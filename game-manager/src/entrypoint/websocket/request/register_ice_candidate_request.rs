use axum::Extension;
use axum::http::StatusCode;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::ice_candidate_response::ICECandidateResponse;

use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::entrypoint::websocket::response::user_response::UserResponse;

use crate::service::ws_session_service::WsSessionService;

#[derive(Deserialize, Debug)]
pub struct RegisterICECandidateRequest {
    candidate: String,
    sdp_mid: Option<String>,
    sdp_m_line_index: Option<i32>,
    to_user_id: i32
}

impl RegisterICECandidateRequest {
    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let ws_session_service = WsSessionService::new(pool.clone());

        let to_user_session = ws_session_service.get_by_user_id(self.to_user_id).await.map_err(status_code_to_string)?;
        let from_user_session = ws_session_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;
        if to_user_session.lobby_id != from_user_session.lobby_id {
            return Err(StatusCode::UNAUTHORIZED.to_string());
        }

        let response = ICECandidateResponse::new(self.candidate.clone(), self.sdp_mid.clone(), self.sdp_m_line_index, UserResponse::from_domain(user));

        connections.send_to_vec_user_id(ResponseEnum::ICECandidate(response), vec![self.to_user_id]).await;

        Ok(())
    }
}
