pub mod cli;
mod entity;
mod routes;
pub mod scrape;

use anyhow::Result;
use cli::Args;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[instrument]
pub async fn run(args: Args) -> Result<()> {
    setup_tracing();

    let app = routes::get_router(args)
        .await?
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on: {}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "seekr_rs=info,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_ids(false)
                .with_file(true)
                .with_line_number(true)
                .with_ansi(true),
        )
        .init();
}
