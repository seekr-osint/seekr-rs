mod embed;
mod language_detection;
mod not_found;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use embed::static_handler;
use entity::people;
use tracing::info;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};

pub async fn get_router() -> anyhow::Result<Router<()>> {
    #[derive(OpenApi)]
    #[openapi(
        paths(language_detection::detect_language_handler,),
        components(schemas(
            language_detection::DetectLanguageQuery,
            language_detection::LanguageDetectionResult,
            language_detection::Language
        ))
    )]
    struct ApiDoc;

    let database_url = std::env::var("DATABASE_URL")?;

    let db = Database::connect(&database_url).await?;
    // .expect("Failed to setup the database");
    Migrator::up(&db, None).await?;
    // .expect("Failed to run migrations for tests");

    Ok(Router::new()
        .route("/", get(static_handler))
        .route("/*file", get(embed::static_handler))
        .route(
            "/api/v1/detect_language",
            post(language_detection::detect_language_handler),
        )
        .route("/api/v1/db", get(db_test))
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .fallback(not_found::not_found_handler)
        .with_state(db))
}

pub async fn db_test(
    State(db): State<DatabaseConnection>,
) -> Result<String, (StatusCode, &'static str)> {
    let person = people::ActiveModel {
        id: Set(0),
        name: Set("greg".to_string()),
        ..Default::default()
    };
    match person.insert(&db).await {
        Ok(_) => Ok(format!("{}", "hello")),
        Err(err) => {
            info!("err: {:?}", err);
            let err: sea_orm::DbErr = err;
            match err {
                sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(error)) => match error {
                    sqlx::Error::Database(e) => {
                        let code = e.code();
                        let res = match code {
                            Some(a) => {
                                let a: String = a.to_string();
                                match a.as_str() {
                                    "1555" => Err((
                                        StatusCode::BAD_REQUEST,
                                        "error person already exists",
                                    )),
                                    _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
                                }
                            }
                            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
                        };
                        res
                    }
                    _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
                },
                _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "unknown error")),
            }
        }
    }
}
