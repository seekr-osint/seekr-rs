use anyhow::Result;
use seekr_rs::run;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(run().await?)
}
