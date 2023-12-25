use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[clap(short, long, default_value = "./seekr.db")]
    db_path: String,

    #[clap(short = 'b', long = "bind", default_value = "127.0.0.1:3000")]
    pub addr: String,
}

impl Args {
    /// touch the database file
    pub fn create_db(&self) -> Result<&Self, std::io::Error> {
        let _ = File::create(&self.db_path)?;
        Ok(self)
    }

    pub fn get_pool(&self) -> String {
        tracing::debug!("database file: {}", self.db_path);
        format!("sqlite:{}", self.db_path)
    }
}
/// Parse cli arguments using clap
pub fn parse() -> Args {
    Args::parse()
}
