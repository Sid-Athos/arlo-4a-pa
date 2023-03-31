use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::init::DatabaseConnection;
use crate::domain::error::internal_error;

pub async fn user_get(DatabaseConnection(conn): DatabaseConnection) -> Result<String, (StatusCode, String)> {
    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}

pub async fn user_post(DatabaseConnection(conn): DatabaseConnection) -> Result<String, (StatusCode, String)> {
    let row = conn
        .query_one("select 1 + 2", &[])
        .await
        .map_err(internal_error)?;
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}

pub fn user_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
    Router::new()
        .route("/", get(user_get).post(user_post))
        .with_state(pool)
}