//! lib of the crate
#![warn(
    clippy::unwrap_used,
    clippy::bool_to_int_with_if,
    clippy::empty_line_after_doc_comments,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::negative_feature_names,
    clippy::panic_in_result_fn,
    clippy::panic,
    clippy::redundant_else,
    clippy::same_name_method,
    clippy::wildcard_imports,
    clippy::mod_module_files,
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    clippy::missing_const_for_fn,
    // clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    clippy::option_if_let_else,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::suspicious_operation_groupings,
    clippy::wrong_self_convention,
    clippy::unused_self,
    clippy::use_debug,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]
#![deny(clippy::unnecessary_self_imports)]
// Compiler warnings
#![forbid(unsafe_code)]
#![warn(
    // missing_docs,
    unknown_lints,
    keyword_idents,
    // missing_copy_implementations,
    // missing_debug_implementations,
    noop_method_call,
    unused_extern_crates,
    unused_import_braces
)]
pub mod seekr {
    use axum::{
        extract::{Query, State},
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
        let id = sqlx::query!(r#"insert into people (firstname) values (?1)"#, query.name)
            .execute(&pool)
            .await?
            .last_insert_rowid();

        let res = format!("<h1>Hello, World!</h1> {}", id);
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

        let res = format!("<h1>{}: {}</h1>", person.id, person.firstname);
        Ok((StatusCode::OK, res))
    }
}
pub mod email;
pub mod embed;
pub mod name;
pub mod named_tensor;
pub mod person;
