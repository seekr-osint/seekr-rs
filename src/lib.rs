pub mod seekr {

    pub struct AppError(anyhow::Error);

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

    // use crate::AppError;

    use axum::response::Html;

    use axum::{
        error_handling::HandleErrorLayer,
        http::StatusCode,
        response::{IntoResponse, Response},
        routing::get,
        BoxError, Router,
    };
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

    use tower_sessions::Session;

    pub async fn handler(_session: Session) -> Html<&'static str> {
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
