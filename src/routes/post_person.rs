use crate::entity::people::{self, ActiveModel};
use axum::response::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode};
use axum::{Form, Json};
use sea_orm::ActiveModelTrait;
use sea_orm::{DatabaseConnection, IntoActiveModel};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::IntoParams;

#[derive(Deserialize, Serialize, IntoParams, Debug)]
pub struct PersonByIDQuery {
    id: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum PostPersonResult {
    AlreadyExists,
    UnknownError,
    Success(u32), // u32 is the id
}
impl IntoResponse for PostPersonResult {
    fn into_response(self) -> Response {
        match self {
            Self::UnknownError => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json("UnknownError")).into_response()
            }
            Self::AlreadyExists => (StatusCode::BAD_REQUEST, Json("NotFound")).into_response(),
            Self::Success(id) => (StatusCode::OK, Json(id)).into_response(),
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/post_person",
    params(PersonByIDQuery),
    responses(
        (status = 200, description = "Success", body = [PostPersonResult]),
        (status = 404, description = "Not found", body = [PostPersonResult], example = json!(PostPersonResult::AlreadyExists)),
        (status = 501, description = "Unknow Error", body = [PostPersonResult], example = json!(PostPersonResult::UnknownError)),
    )
)]
#[instrument]
/// Post Person
pub async fn post_person_handler(
    State(db): State<DatabaseConnection>,
    Json(person): Json<people::Model>,
) -> PostPersonResult {
    match ActiveModel::from(person).insert(&db).await {
        Ok(model) => PostPersonResult::Success((model as people::Model).id),
        Err(_err) => PostPersonResult::UnknownError,
    }
    //         info!("err: {:?}", err);
    //         let err: sea_orm::DbErr = err;
    //         match err {
    //             sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(error)) => match error {
    //                 sqlx::Error::Database(e) => {
    //                     let code = e.code();
    //                     let res = match code {
    //                         Some(a) => {
    //                             let a: String = a.to_string();
    //                             match a.as_str() {
    //                                 "1555" => Err((
    //                                     StatusCode::BAD_REQUEST,
    //                                     "error person already exists",
    //                                 )),
    //                                 _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
    //                             }
    //                         }
    //                         _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
    //                     };
    //                     res
    //                 }
    //                 _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
    //             },
    //             _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
    //         }
    //     }
    // }
}
