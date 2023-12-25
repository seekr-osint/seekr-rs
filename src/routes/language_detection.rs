use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use axum::http::StatusCode;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DetectLanguageQuery {
    #[schema(example = "Hello world. This is some english text for testing.")]
    text: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum Language {
    Afrikaans,
    Albanian,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bokmal,
    Bosnian,
    Bulgarian,
    Catalan,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Ganda,
    Georgian,
    German,
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Kazakh,
    Korean,
    Latin,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Maori,
    Marathi,
    Mongolian,
    Nynorsk,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romanian,
    Russian,
    Serbian,
    Shona,
    Slovak,
    Slovene,
    Somali,
    Sotho,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Tamil,
    Telugu,
    Thai,
    Tsonga,
    Tswana,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Welsh,
    Xhosa,
    Yoruba,
    Zulu,
}

// FIXME this is kinda a hack to get the api docs workings without inheritance due to rust
// limitations on implementing traits on external enums
impl From<lingua::Language> for Language {
    fn from(value: lingua::Language) -> Self {
        match value {
            lingua::Language::Afrikaans => Language::Afrikaans,
            lingua::Language::Albanian => Language::Albanian,
            lingua::Language::Arabic => Language::Arabic,
            lingua::Language::Armenian => Language::Armenian,
            lingua::Language::Azerbaijani => Language::Azerbaijani,
            lingua::Language::Basque => Language::Basque,
            lingua::Language::Belarusian => Language::Belarusian,
            lingua::Language::Bengali => Language::Bengali,
            lingua::Language::Bokmal => Language::Bokmal,
            lingua::Language::Bosnian => Language::Bosnian,
            lingua::Language::Bulgarian => Language::Bulgarian,
            lingua::Language::Catalan => Language::Catalan,
            lingua::Language::Chinese => Language::Chinese,
            lingua::Language::Croatian => Language::Croatian,
            lingua::Language::Czech => Language::Czech,
            lingua::Language::Danish => Language::Danish,
            lingua::Language::Dutch => Language::Dutch,
            lingua::Language::English => Language::English,
            lingua::Language::Esperanto => Language::Esperanto,
            lingua::Language::Estonian => Language::Estonian,
            lingua::Language::Finnish => Language::Finnish,
            lingua::Language::French => Language::French,
            lingua::Language::Ganda => Language::Ganda,
            lingua::Language::Georgian => Language::Georgian,
            lingua::Language::German => Language::German,
            lingua::Language::Greek => Language::Greek,
            lingua::Language::Gujarati => Language::Gujarati,
            lingua::Language::Hebrew => Language::Hebrew,
            lingua::Language::Hindi => Language::Hindi,
            lingua::Language::Hungarian => Language::Hungarian,
            lingua::Language::Icelandic => Language::Icelandic,
            lingua::Language::Indonesian => Language::Indonesian,
            lingua::Language::Irish => Language::Irish,
            lingua::Language::Italian => Language::Italian,
            lingua::Language::Japanese => Language::Japanese,
            lingua::Language::Kazakh => Language::Kazakh,
            lingua::Language::Korean => Language::Korean,
            lingua::Language::Latin => Language::Latin,
            lingua::Language::Latvian => Language::Latvian,
            lingua::Language::Lithuanian => Language::Lithuanian,
            lingua::Language::Macedonian => Language::Macedonian,
            lingua::Language::Malay => Language::Malay,
            lingua::Language::Maori => Language::Maori,
            lingua::Language::Marathi => Language::Marathi,
            lingua::Language::Mongolian => Language::Mongolian,
            lingua::Language::Nynorsk => Language::Nynorsk,
            lingua::Language::Persian => Language::Persian,
            lingua::Language::Polish => Language::Polish,
            lingua::Language::Portuguese => Language::Portuguese,
            lingua::Language::Punjabi => Language::Punjabi,
            lingua::Language::Romanian => Language::Romanian,
            lingua::Language::Russian => Language::Russian,
            lingua::Language::Serbian => Language::Serbian,
            lingua::Language::Shona => Language::Shona,
            lingua::Language::Slovak => Language::Slovak,
            lingua::Language::Slovene => Language::Slovene,
            lingua::Language::Somali => Language::Somali,
            lingua::Language::Sotho => Language::Sotho,
            lingua::Language::Spanish => Language::Spanish,
            lingua::Language::Swahili => Language::Swahili,
            lingua::Language::Swedish => Language::Swedish,
            lingua::Language::Tagalog => Language::Tagalog,
            lingua::Language::Tamil => Language::Tamil,
            lingua::Language::Telugu => Language::Telugu,
            lingua::Language::Thai => Language::Thai,
            lingua::Language::Tsonga => Language::Tsonga,
            lingua::Language::Tswana => Language::Tswana,
            lingua::Language::Turkish => Language::Turkish,
            lingua::Language::Ukrainian => Language::Ukrainian,
            lingua::Language::Urdu => Language::Urdu,
            lingua::Language::Vietnamese => Language::Vietnamese,
            lingua::Language::Welsh => Language::Welsh,
            lingua::Language::Xhosa => Language::Xhosa,
            lingua::Language::Yoruba => Language::Yoruba,
            lingua::Language::Zulu => Language::Zulu,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum LanguageDetectionResult {
    Error(String),
    Language(Language),
}

#[utoipa::path(
    post,
    path = "/api/v1/detect_language",
    request_body = DetectLanguageQuery,
    responses(
        (status = 200, description = "Language detection success", body = [LanguageDetectionResult], example = json!(LanguageDetectionResult::Language(Language::English))),

        (status = 501, description = "Error Language not detected", body = [LanguageDetectionResult], example = json!(LanguageDetectionResult::Error(String::from("language not detected")))),
    )
)]
#[instrument]
pub async fn detect_language_handler(
    Json(payload): Json<DetectLanguageQuery>,
) -> (StatusCode, impl IntoResponse) {
    let detector = lingua::LanguageDetectorBuilder::from_all_languages().build();
    match detector.detect_language_of(payload.text) {
        Some(language) => (
            StatusCode::OK,
            Json(LanguageDetectionResult::Language(Language::from(language))),
        ),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LanguageDetectionResult::Error(
                "language not detected".to_string(),
            )),
        ),
    }
}
