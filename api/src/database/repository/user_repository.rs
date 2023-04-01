use axum::http::StatusCode;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::entity::user_entity::UserEntity;
use crate::database::mapper::user_entity_mapper::UserEntityMapper;
use crate::domain::error::internal_error;
use crate::domain::model::user::User;
use crate::entrypoint::command::create_user_command::CreateUserCommand;

pub struct UserRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl UserRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        UserRepository {
            connection
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, (StatusCode, String)> {

        let conn = self.connection.get().await.map_err(internal_error)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.user WHERE id = $1", &[&user_id])
            .await
            .map_err(internal_error)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, (StatusCode, String)> {

        let conn = self.connection.get().await.map_err(internal_error)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.user WHERE email = $1", &[&email])
            .await
            .map_err(internal_error)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn create_user(&self, user: CreateUserCommand) -> Result<User, (StatusCode, String)> {

        let conn = self.connection.get().await.map_err(internal_error)?;

        let row = conn
            .query_one("INSERT INTO coding_games.user (pseudo, email, password) VALUES ($1, $2, $3) RETURNING *", &[&user.pseudo, &user.email, &user.password])
            .await
            .map_err(internal_error)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }
}
