use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::ranking::route::request::ranking_request::RankingRequest;
use crate::entrypoint::ranking::route::request::update_ranking_request::UpdateRankingRequest;
use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;
use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::ranking_service::RankingService;
use crate::service::user_service::UserService;

#[utoipa::path(
    put,
    path = "/ranking/",
    request_body = UpdateRankingRequest,
    responses(
        (status = 200, description = "Ranking updated", body = RankingResponse,),
    ),
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    ),
    tag="ranking"
)]
pub async fn update_ranking(State(pool): State<ConnectionPool>, Json(ranking): Json<UpdateRankingRequest>) -> Result<Json<Vec<RankingResponse>>, StatusCode> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone())
    );

    let rankings = ranking_service.update_ranking2(ranking.game_id,ranking.p1_id,ranking.p2_id,ranking.tie,ranking.winner_id).await?;

    Ok(Json(RankingResponse::from_vec_domain(rankings)))
}