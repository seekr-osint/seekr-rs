use anyhow::Result;
use seekr_rs::{cli, run};

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::parse();
    Ok(run(args).await?)
}
