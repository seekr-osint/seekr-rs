pub mod cli;
pub mod people;
pub mod routes;
pub mod scrape;
pub mod users;
pub mod web;

use anyhow::Result;
use cli::Args;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[instrument]
pub async fn run(args: Args) -> Result<()> {
    setup_tracing();

    let app = routes::get_router(&args)
        .await?
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(args.addr).await?;
    info!("listening on: {}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "seekr=info,axum_login=debug,tower_http=debug,axum::rejection=trace".into()
            }),
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
