use tokio_postgres::Row;

pub struct UserEntity {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
}

impl UserEntity {
    pub fn new(row: Row) -> Self {
        UserEntity {
            id: row.get("id"),
            pseudo: row.get("pseudo"),
            email: row.get("email"),
            password: row.get("password"),
            admin: row.get("admin"),
        }
    }
}
