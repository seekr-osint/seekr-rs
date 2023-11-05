use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    BoxError, Router,
};
use std::net::SocketAddr;
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{
    // session_store::ExpiredDeletion,
    sqlx::SqlitePool,
    Expiry,
    Session,
    SessionManagerLayer,
    // Expiry, Session, SessionManagerLayer,
    SqliteStore,
};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

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
        .route("/", get(handler::handler))
        .route("/error", get(handler::test_handler))
        .layer(session_service);

    let addr = SocketAddr::from(addr_arg);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

mod handler {
    use crate::AppError;

    use axum::{
        error_handling::HandleErrorLayer,
        http::StatusCode,
        response::{Html, IntoResponse, Response},
        routing::get,
        BoxError, Router,
    };
    use std::net::SocketAddr;
    use time::Duration;
    use tower::ServiceBuilder;
    use tower_sessions::{
        // session_store::ExpiredDeletion,
        sqlx::SqlitePool,
        Expiry,
        Session,
        SessionManagerLayer,
        // Expiry, Session, SessionManagerLayer,
        SqliteStore,
    };
    use utoipa::OpenApi;
    use utoipa_redoc::{Redoc, Servable};

    pub async fn handler(session: Session) -> Html<&'static str> {
        Html("<h1>Hello, World!</h1>")
    }
    pub async fn test_handler() -> Result<(), AppError> {
        try_thing()?;
        Ok(())
    }

    fn try_thing() -> Result<(), anyhow::Error> {
        anyhow::bail!("it failed!")
    }
}
