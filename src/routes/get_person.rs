use crate::entity::people;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::IntoParams;

#[derive(Deserialize, Serialize, IntoParams, Debug)]
pub struct PersonByIDQuery {
    id: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum PersonByIDQueryError {
    NotFound,
    UnknownError,
}
impl IntoResponse for PersonByIDQueryError {
    fn into_response(self) -> Response {
        match self {
            Self::UnknownError => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json("UnknownError")).into_response()
            }
            Self::NotFound => (StatusCode::NOT_FOUND, Json("NotFound")).into_response(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/get_person",
    params(PersonByIDQuery),
    responses(
        (status = 200, description = "Success", body = [Model]),
        (status = 404, description = "Not found", body = [PersonByIDQueryError], example = json!(PersonByIDQueryError::NotFound)),
        (status = 501, description = "Unknow Error", body = [PersonByIDQueryError], example = json!(PersonByIDQueryError::UnknownError)),
    )
)]
#[instrument]
pub async fn get_person_handler(
    query: Query<PersonByIDQuery>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<people::Model>, PersonByIDQueryError> {
    info!("get id: {}", query.id);
    if let Ok(people) = people::Entity::find_by_id(query.id).one(&db).await {
        match people {
            Some(people) => Ok(Json(people)),
            None => Err(PersonByIDQueryError::NotFound),
        }
    } else {
        Err(PersonByIDQueryError::UnknownError)
    }
}
