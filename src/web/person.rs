use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::people::{get_person, GetPersonError, Person};
use crate::users::AuthSession;

#[derive(Debug, Template)]
#[template(path = "person.html")]
struct PersonTemplate<'a> {
    name: &'a str,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetPersonProtectedQuery {
    #[schema(example = 4u32)]
    pub id: u32,
}

#[derive(Debug, Error)]
pub enum GetPersonProtectedError {
    #[error("getting person: {0}")]
    GetPerson(#[from] GetPersonError),
    #[error("unknown")]
    Unknown,
}

impl IntoResponse for GetPersonProtectedError {
    fn into_response(self) -> Response {
        format!("error: {}", self).into_response()
    }
}

pub mod get {
    use super::*;
    pub async fn person_protected(
        query: Query<GetPersonProtectedQuery>,
        auth_session: AuthSession,
    ) -> Result<impl IntoResponse, GetPersonProtectedError> {
        let person = get_person(auth_session, query.id).await?;
        Ok(PersonTemplate { name: &person.name }.into_response())
    }
}
