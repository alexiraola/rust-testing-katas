use std::sync::Mutex;

use async_trait::async_trait;
use rusqlite::{params, Connection};

use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    value_objects::{id::Id, password::Password},
};

#[derive(Debug)]
pub struct Sqlite {
    connection: Mutex<rusqlite::Connection>,
}

impl Sqlite {
    pub async fn new(path: &str) -> anyhow::Result<Sqlite> {
        let connection = Connection::open(path)?;

        let create_schema = "create table if not exists users
        (
            id TEXT PRIMARY KEY NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL
        );";

        connection.execute(create_schema, ())?;

        Ok(Sqlite {
            connection: Mutex::new(connection),
        })
    }
}

#[async_trait]
impl UserRepository for Sqlite {
    async fn save(&self, user: crate::domain::entities::user::User) -> Result<(), String> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .execute(
                "INSERT INTO users (id, email, password) VALUES (?1, ?2, ?3)",
                (&user.id(), &user.email(), &user.password()),
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_email(
        &self,
        email: crate::domain::value_objects::email::Email,
    ) -> Result<Option<crate::domain::entities::user::User>, String> {
        let user = self
            .connection
            .lock()
            .map_err(|e| e.to_string())?
            .query_row(
                "SELECT * FROM users where email=:email",
                params![email.to_string()],
                |row| {
                    let id: String = row.get(0)?;
                    let email: String = row.get(1)?;
                    let password: String = row.get(2)?;

                    Ok(User::new(
                        id.try_into().unwrap(),
                        email.try_into().unwrap(),
                        Password::from_hash(password),
                    ))
                },
            );

        match user {
            Ok(u) => Ok(Some(u)),
            Err(error) => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<crate::domain::entities::user::User>, String> {
        todo!()
    }

    async fn remove(&self, user: crate::domain::entities::user::User) -> Result<(), String> {
        todo!()
    }
}
