// use crate::entity::people::{self, ActiveModel};
// use axum::response::{IntoResponse, Response};
// use axum::Json;
// use axum::{extract::State, http::StatusCode};
// use sea_orm::ActiveModelTrait;
// use sea_orm::DatabaseConnection;
// use serde::{Deserialize, Serialize};
// use tracing::{info, instrument};

// #[derive(Serialize, Deserialize, Debug)]
// pub enum PostPersonResult {
//     AlreadyExists,
//     UnknownError,
//     Success(u32), // u32 is the id
// }

// impl IntoResponse for PostPersonResult {
//     fn into_response(self) -> Response {
//         match self {
//             Self::UnknownError => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, Json("UnknownError")).into_response()
//             }
//             Self::AlreadyExists => (StatusCode::BAD_REQUEST, Json("NotFound")).into_response(),
//             Self::Success(id) => (StatusCode::OK, Json(id)).into_response(),
//         }
//     }
// }

// #[utoipa::path(
//     post,
//     path = "/api/v1/post_person",
//     request_body = Model,
//     responses(
//         (status = 200, description = "Success", body = [PostPersonResult]),
//         (status = 404, description = "Not found", body = [PostPersonResult], example = json!(PostPersonResult::AlreadyExists)),
//         (status = 501, description = "Unknow Error", body = [PostPersonResult], example = json!(PostPersonResult::UnknownError)),
//     )
// )]
// #[instrument]
// /// Post Person
// pub async fn post_person_handler(
//     State(db): State<DatabaseConnection>,
//     Json(person): Json<people::Model>,
// ) -> PostPersonResult {
//     match ActiveModel::from(person).insert(&db).await {
//         Ok(model) => PostPersonResult::Success((model as people::Model).id),
//         Err(err) => {
//             info!("err: {:?}", err);
//             if let sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sqlx::Error::Database(e))) =
//                 err
//             {
//                 info!("err: {:?}", e);
//                 match e.code() {
//                     Some(code) if code == "1555" => PostPersonResult::AlreadyExists,
//                     _ => PostPersonResult::UnknownError,
//                 }
//             } else {
//                 PostPersonResult::UnknownError
//             }
//         }
//     }
// }
