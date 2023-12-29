pub mod embed;
pub mod get_person;
pub mod language_detection;
pub mod list_people;
pub mod not_found;
pub mod post_person;

use crate::cli::Args;
use axum::{
    error_handling::HandleErrorLayer,
    // extract::State,
    http::StatusCode,
    BoxError,
    Router,
};
use utoipa::OpenApi;
// use tracing::info;
use sqlx::SqlitePool;
use time::Duration;
use tower::ServiceBuilder;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    users::Backend,
    web::{auth, protected},
};

use axum_login::{
    login_required,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};

pub async fn get_router(args: &Args) -> anyhow::Result<Router<()>> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            language_detection::detect_language_handler,
            // list_people::list_people_handler,
            // get_person::get_person_handler,
            // post_person::post_person_handler,
        ),
        components(schemas(
            language_detection::DetectLanguageQuery,
            language_detection::LanguageDetectionResult,
            language_detection::Language,
            // Model,
        ))
    )]
    struct ApiDoc;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let sqlx_db = SqlitePool::connect(&args.create_db()?.get_pool()).await?;
    sqlx::migrate!().run(&sqlx_db).await?;
    let backend = Backend::new(sqlx_db);

    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

    let app = protected::router()
        .route_layer(login_required!(Backend, login_url = "/login"))
        .merge(auth::router())
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .layer(auth_service);
    Ok(app)

    // Ok(Router::new()
    //     .route("/", get(static_handler))
    //     .route("/*file", get(embed::static_handler))
    //     .route(
    //         "/api/v1/detect_language",
    //         post(language_detection::detect_language_handler),
    //     )
    //     // .route("/api/v1/db", get(db_test))
    //     .route("/api/v1/list_people", get(list_people::list_people_handler))
    //     .route("/api/v1/get_person", get(get_person::get_person_handler))
    //     .route(
    //         "/api/v1/post_person",
    //         post(post_person::post_person_handler),
    //     )
    //     .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
    //     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    //     .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    //     .fallback(not_found::not_found_handler)
    //     .with_state(db))
}
