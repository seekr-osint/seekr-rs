pub mod seekr {

    pub struct AppError(anyhow::Error);

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self.0),
            )
                .into_response()
        }
    }

    impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
    {
        fn from(err: E) -> Self {
            Self(err.into())
        }
    }

    use std::fmt::format;

    use axum::async_trait;
    use axum::extract::{FromRef, FromRequestParts, Query, State};
    use axum::http::request::Parts;
    use axum::response::Html;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use serde::Deserialize;
    use sqlx::sqlite::SqlitePool;
    use sqlx::Error;
    use tower_sessions::Session;
    use utoipa::{IntoParams, IntoResponses, ToSchema};

    #[derive(Deserialize, IntoParams)]
    pub struct PersonByIDQuery {
        /// Search by value. Search is incase sensitive.
        id: i64,
    }
    pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Sqlite>);

    fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
    {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
    #[async_trait]
    impl<S> FromRequestParts<S> for DatabaseConnection
    where
        SqlitePool: FromRef<S>,
        S: Send + Sync,
    {
        type Rejection = (StatusCode, String);

        async fn from_request_parts(
            _parts: &mut Parts,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
            let pool = SqlitePool::from_ref(state);

            let conn = pool.acquire().await.map_err(internal_error)?;

            Ok(Self(conn))
        }
    }

    #[derive(Debug, IntoResponses, ToSchema)]
    #[response(status = 200, description = "Return the person")]
    pub struct Person {
        name: String,
    }
    #[utoipa::path(
        get,
                path = "/api/v1/person",
                        params(
           PersonByIDQuery
        ),
        responses(
            (status = 200, description = "Found", body = [Person])
        ))]
    pub async fn get_person(
        // pool: SqlitePool,
        query: Query<PersonByIDQuery>,
        DatabaseConnection(mut conn): DatabaseConnection,
    ) -> Result<impl IntoResponse, AppError>
// Result<Html<&'dyn str>, AppError>
    {
        // let mut tx = conn.begin().await?;
        // let id = sqlx::query_scalar!(
        // r#"
        // INSERT INTO PEOPLE ( NAME )
        // VALUES ( $1 )
        // "#,
        // "hacker")
        // .execute(conn)
        // .await?
        // .last_insert_rowid();
        // format!("test")
        // let recs = sqlx::query!("SELECT ID, NAME FROM PEOPLE")
        //     .fetch_all(&pool)
        //     .await?;
        // for rec in recs {
        //     println!("- {}: {}", rec.ID, &rec.NAME,);
        // }

        let res = format!("<h1>Hello, World!</h1> {}", "id");
        Ok(res)
    }

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
