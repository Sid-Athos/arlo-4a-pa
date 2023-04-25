use axum::{Router, routing::get, response::sse::{Event, KeepAlive, Sse}, Json};
use std::{time::Duration, convert::Infallible};
use std::ops::Deref;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use tokio_stream::StreamExt as _ ;
use futures_util::stream::{self, Stream};
use serde::Serialize;
use serde_json::json;
use tracing_subscriber::fmt::format;
use crate::database::init::ConnectionPool;
use crate::database::repository::ranking_repository::RankingRepository;
use crate::RankingResponse;
use crate::service::ranking_service::RankingService;

#[utoipa::path(
    get,
    path = "/ranking/{game_id}",
    params(
        ("user_id" = String,),
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse,),
        (status = 404, description = "User not found",),
    )
)]
pub async unsafe fn get_ranking_by_game_id(State(pool): State<ConnectionPool>, Path(game_id): Path<i32>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let ranking_service = RankingService::new(
        RankingRepository::new(pool.clone()),
    );

    let stream = stream::repeat_with(async ||
        Event::default().json_data(
            serde_json::to_string(&(RankingResponse::from_vec_domain(ranking_service.get_ranking_by_game(game_id).await.unwrap()))).unwrap())
        ).map(Ok)
        .throttle(Duration::from_secs(3));


    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(3))
            .text("keep-alive-text"),
    )

}