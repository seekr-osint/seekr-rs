mod embed;
mod language_detection;
mod not_found;

use axum::{
    routing::{get, post},
    Router,
};
use embed::static_handler;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

pub fn get_router() -> Router<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            language_detection::detect_language_handler,
        ),
        components(
            schemas(
                language_detection::DetectLanguageQuery,
                language_detection::LanguageDetectionResult,
                language_detection::Language
            )
        ),
        // modifiers(&SecurityAddon),
        // tags(
        //     (name = "todo", description = "Todo items management API")
        // )
    )]
    struct ApiDoc;

    Router::new()
        .route("/", get(static_handler))
        .route("/*file", get(embed::static_handler))
        .route(
            "/api/v1/detect_language",
            post(language_detection::detect_language_handler),
        )
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
        .fallback(not_found::not_found_handler)
}
