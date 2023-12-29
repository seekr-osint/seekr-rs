use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlxinsert::SqliteInsert;
use thiserror::Error;
use utoipa::ToSchema;

use crate::users::AuthSession;

#[derive(Clone, Serialize, Deserialize, FromRow, ToSchema, SqliteInsert)]
pub struct Person {
    #[schema(example = 4u32)]
    pub id: u32,

    #[schema(example = "greg")]
    pub name: String,

    #[schema(example = "seekr")]
    pub owner: String,
}

// /// Used in requests creating a person
// #[derive(Clone, Serialize, Deserialize, FromRow, ToSchema)]
// pub struct PersonBuilder {
//     #[schema(example = "greg")]
//     pub name: String,
// }

pub async fn insert_person(
    auth_session: AuthSession,
    person: Person,
) -> Result<Person, InsertPersonError> {
    match auth_session.user {
        Some(user) => {
            let mut person = person;
            person.owner = user.username;
            let db = auth_session.backend.get_pool();
            let _ = person.insert_raw(&db, "people").await?;
            Ok(person)
        }
        None => Err(InsertPersonError::Auth),
    }
}

pub async fn get_person(auth_session: AuthSession, id: u32) -> Result<Person, GetPersonError> {
    match auth_session.user {
        Some(user) => {
            let db = auth_session.backend.get_pool();
            if let Some(person) = sqlx::query_as("select * from people where id = ? and owner = ?")
                .bind(id)
                .bind(&user.username)
                .fetch_optional(&db)
                .await?
            {
                Ok(person)
            } else {
                Err(GetPersonError::NotFound {
                    id,
                    owner: user.username,
                })
            }
        }
        None => Err(GetPersonError::Auth),
    }
}

#[derive(Debug, Error)]
pub enum GetPersonError {
    #[error("Person not found. ID: {id:?} owner: {owner:?}")]
    NotFound { id: u32, owner: String },

    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),

    #[error("not authenticated")]
    Auth,
}

#[derive(Debug, Error)]
pub enum InsertPersonError {
    #[error("sqlx error")]
    Sqlxinsert(#[from] eyre::ErrReport),

    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),

    #[error("not authenticated")]
    Auth,
}
