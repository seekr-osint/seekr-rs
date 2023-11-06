pub mod seekr {
    use axum::extract::{Query, State};
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use serde::Deserialize;
    use sqlx::sqlite::SqlitePool;
    use utoipa::IntoParams;
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

    #[allow(dead_code)]
    #[derive(Deserialize, IntoParams)]
    pub struct CreatePersonQuery {
        id: Option<i64>,
        name: String,
    }

    pub async fn post_person(
        query: Query<CreatePersonQuery>,
        State(pool): State<SqlitePool>,
    ) -> Result<impl IntoResponse, AppError> {
        let id = sqlx::query!(r#"insert into people (name) values (?1)"#, query.name)
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
