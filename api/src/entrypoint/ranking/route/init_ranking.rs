use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::ranking::route::request::ranking_request::RankingRequest;
use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;
use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::ranking_service::RankingService;
use crate::service::user_service::UserService;

#[utoipa::path(
    post,
    path = "/ranking/init",
    request_body = RankingRequest,
    responses(
        (status = 200, description = "Ranking initialised", body = RankingResponse,),
        (status = 409, description = "Ranking already init",),
    ),
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    )
)]
pub async fn init_ranking(State(pool): State<ConnectionPool>, Json(ranking): Json<RankingRequest>) -> Result<Json<RankingResponse>, StatusCode> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone())
    );

    let ranking = ranking_service.init_ranking(ranking.user_id,ranking.game_id).await?;

    Ok(Json(RankingResponse::from_domain(ranking)))
}