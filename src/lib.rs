pub mod seekr {
    // pub struct DbError(sqlx::Error);

    // impl From<sqlx::Error> for DbError {
    //     fn from(error: sqlx::Error) -> Self {
    //         Self(error)
    //     }
    // }

    // impl IntoResponse for DbError {
    //     fn into_response(self) -> axum::response::Response {
    //         println!("ERROR: {}", self.0);
    //         (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
    //     }
    // }

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

    // use std::fmt::format;

    // use axum::async_trait;
    use axum::extract::{FromRef, FromRequestParts, Query, State};
    // use axum::http::request::Parts;
    // use axum::response::Html;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use serde::Deserialize;
    use sqlx::sqlite::SqlitePool;
    // use sqlx::Error;
    // use tower_sessions::Session;
    use utoipa::IntoParams;

    // pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Sqlite>);

    // fn db_error<E>(err: E) -> (StatusCode, String)
    // where
    //     E: std::error::Error,
    // {
    //     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    // }

    // #[async_trait]
    // impl<S> FromRequestParts<S> for DatabaseConnection
    // where
    //     SqlitePool: FromRef<S>,
    //     S: Send + Sync,
    // {
    //     // type Rejection = (StatusCode, String);
    //     type Rejection = AppError;

    //     async fn from_request_parts(
    //         _parts: &mut Parts,
    //         state: &S,
    //     ) -> Result<Self, Self::Rejection> {
    //         let pool = SqlitePool::from_ref(state);

    //         let conn = pool.acquire().await.map_err(Self::Rejection::from)?;

    //         Ok(Self(conn))
    //     }
    // }

    // #[derive(Debug, IntoResponses, ToSchema)]
    // #[response(status = 200, description = "Return the person")]
    // pub struct Person {
    //     name: String,
    // }
    // #[utoipa::path(
    //     get,
    //             path = "/api/v1/person",
    //                     params(
    //        PersonByIDQuery
    //     ),
    //     responses(
    //         (status = 200, description = "Found", body = [Person])
    //     ))]

    #[derive(Deserialize, IntoParams)]
    pub struct Person {
        id: Option<i64>,
        name: String,
    }
    pub async fn post_person(
        query: Query<Person>,
        State(pool): State<SqlitePool>,
    ) -> Result<impl IntoResponse, AppError> {
        let id = sqlx::query!(
            r#"
        insert into people (name) values (?1)"#,
            query.name
        )
        .execute(&pool)
        .await?
        .last_insert_rowid();

        let res = format!("<h1>Hello, World!</h1> {}", id);
        println!("{}", res);
        Ok(res)
    }

    #[derive(Deserialize, IntoParams)]
    pub struct PersonByIDQuery {
        id: i64,
    }
    pub async fn get_person(
        query: Query<PersonByIDQuery>,
        State(pool): State<SqlitePool>,
    ) -> Result<(StatusCode, String), AppError> {
        // TODO Option
        let person = sqlx::query!("select * from people where id = ?1", query.id)
            .fetch_one(&pool)
            .await?;

        let res = format!("<h1>{}: {}</h1>", person.id, person.name);
        Ok((StatusCode::OK, res))
    }
}
