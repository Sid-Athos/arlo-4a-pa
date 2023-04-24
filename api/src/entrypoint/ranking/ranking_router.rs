use axum::Router;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub fn ranking_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .with_state(pool)

}