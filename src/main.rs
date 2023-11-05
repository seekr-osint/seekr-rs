use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    BoxError, Router,
};
use seekr::seekr;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{
    // session_store::ExpiredDeletion,
    sqlx::SqlitePool,
    Expiry,
    SessionManagerLayer,
    // Expiry, Session, SessionManagerLayer,
    SqliteStore,
};
use utoipa::OpenApi;
use utoipa::{IntoParams, ToSchema};
use utoipa_redoc::{Redoc, Servable};

// #[derive(Serialize, Deserialize, ToSchema)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;
    // TODO command line arguments
    let addr_arg = ([127, 0, 0, 1], 3000);

    let pool_arg = "sqlite:seekr.db";

    let pool = SqlitePool::connect(pool_arg).await?;
    let session_store = SqliteStore::new(pool);
    session_store.migrate().await?;

    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(
            SessionManagerLayer::new(session_store)
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(Duration::seconds(10))),
        );

    let app = Router::new()
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", get(seekr::handler))
        .route("/error", get(seekr::test_handler))
        .layer(session_service);

    let addr = SocketAddr::from(addr_arg);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
