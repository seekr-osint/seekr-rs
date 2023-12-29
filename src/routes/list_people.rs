// use crate::entity::people;
// use axum::Json;
// use axum::{extract::State, http::StatusCode};
// use sea_orm::DatabaseConnection;
// use sea_orm::EntityTrait;
// use tracing::instrument;

// #[utoipa::path(
//     get,
//     path = "/api/v1/list_people",
//     responses(
//         (status = 200, description = "Language detection success", body = [Vec<Model>],content_type = "application/json",),
//     )
// )]
// #[instrument]
// /// List all people in the Database
// pub async fn list_people_handler(
//     State(db): State<DatabaseConnection>,
// ) -> Result<Json<Vec<people::Model>>, (StatusCode, &'static str)> {
//     if let Ok(people) = people::Entity::find().all(&db).await {
//         Ok(Json(people))
//     } else {
//         Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error"))
//     }
// }
