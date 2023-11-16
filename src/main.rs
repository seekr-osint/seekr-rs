use ::seekr::embed;
use axum::{routing, Router};
use seekr::seekr;
use sqlx::SqlitePool;
use std::fs::File;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::instrument;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[derive(Debug)]
pub struct Args<'a> {
    addr: ([u8; 4], u16),
    db_path: &'a str,
}
impl Args<'_> {
    #[instrument]
    pub fn create_db(&self) -> Result<&Self, anyhow::Error> {
        let _ = File::create(self.db_path)?;
        Ok(self)
    }

    #[instrument]
    pub fn get_pool(&self) -> String {
        tracing::debug!("database file: {}", self.db_path);
        format!("sqlite:{}", self.db_path)
    }
}

#[instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "seekr=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;
    // TODO command line arguments
    let args = Args {
        addr: ([127, 0, 0, 1], 3000),
        db_path: "seekr.db",
    };

    let pool = SqlitePool::connect(&args.create_db()?.get_pool()).await?;
    sqlx::migrate!().run(&pool).await?;

    let app = Router::new()
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", routing::get(embed::static_handler))
        .route("/*file", routing::get(embed::static_handler))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .route("/api/v1/person", routing::get(seekr::get_person))
        .route("/api/v1/person2", routing::get(seekr::post_person))
        .with_state(pool);

    let addr = SocketAddr::from(args.addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
