use axum::extract::Path;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::init::DatabaseConnection;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::internal_error;

pub async fn user_get(DatabaseConnection(conn): DatabaseConnection, Path(user_id): Path<i32>) -> Result<String, (StatusCode, String)> {
    let user_repository = UserRepository::new(conn);

    let user = user_repository.get_user_by_id(user_id).await?;

    serde_json::to_string(&user).map_err(internal_error)
}

pub fn user_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
    Router::new()
        .route("/:user_id", get(user_get))
        .with_state(pool)
}