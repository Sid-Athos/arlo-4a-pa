use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_duplicate_key, database_error_not_found, DatabaseError};
use crate::database::entity::user_entity::UserEntity;
use crate::database::mapper::user_entity_mapper::UserEntityMapper;
use crate::domain::model::user::User;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::command::update_user_command::UpdateUserCommand;

pub struct UserRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl UserRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        UserRepository {
            connection
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.user WHERE id = $1", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.user WHERE email = $1", &[&email])
            .await
            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn get_user_by_pseudo(&self, pseudo: String) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("SELECT * FROM coding_games.user WHERE pseudo = $1", &[&pseudo])
                .await
                .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn create_user(&self, user: CreateUserCommand) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.user (pseudo, email, password) VALUES ($1, $2, $3) RETURNING *", &[&user.pseudo, &user.email, &user.password])
            .await
            .map_err(database_error_duplicate_key)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn search_user(&self, search: String) -> Result<Vec<User>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.user WHERE pseudo LIKE $1", &[&format!("%{}%", search)])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(UserEntityMapper::entity_to_domain(UserEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn delete_account(&self, user_id: i32) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("DELETE FROM coding_games.user WHERE id = $1 RETURNING *", &[&user_id])
                            .await
                            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn change_password(&self, user_id: i32, password: String) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.user SET password = $1 WHERE id = $2 RETURNING *", &[&password, &user_id])
                            .await
                            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn update_user(&self, update_user_command: UpdateUserCommand) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.user SET pseudo = $1 WHERE id = $2 RETURNING *", &[&update_user_command.pseudo, &update_user_command.id])
                            .await
                            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn.query("SELECT * FROM coding_games.user", &[])
                                .await
                                .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(UserEntityMapper::entity_to_domain(UserEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn give_admin_role(&self, user_id: i32) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.user SET admin = true WHERE id = $1 RETURNING *", &[&user_id])
                            .await
                            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }

    pub async fn remove_admin_role(&self, user_id: i32) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.user SET admin = false WHERE id = $1 RETURNING *", &[&user_id])
                            .await
                            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }
}
