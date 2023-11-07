pub mod embed {
    use axum::{
        http::{header, StatusCode, Uri},
        response::{Html, IntoResponse, Response},
    };
    use rust_embed::RustEmbed;

    use tracing::instrument;

    #[instrument]
    pub async fn static_handler(uri: Uri) -> impl IntoResponse {
        let path = uri.path().trim_start_matches('/').to_string();
        match path.as_str() {
            "" => StaticFile("index.html".to_string()),
            _ => StaticFile(path),
        }
    }

    pub async fn not_found() -> Html<&'static str> {
        Html("<h1>404</h1><p>Not Found</p>")
    }

    #[derive(RustEmbed, Debug)]
    #[folder = "web"]
    struct Asset;
    pub struct StaticFile<T>(pub T);

    impl<T> IntoResponse for StaticFile<T>
    where
        T: Into<String>,
    {
        fn into_response(self) -> Response {
            let path = self.0.into();

            match Asset::get(path.as_str()) {
                Some(content) => {
                    let mime = mime_guess::from_path(path).first_or_octet_stream();
                    ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
                }
                None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            }
        }
    }
}
