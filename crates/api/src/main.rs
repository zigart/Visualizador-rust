#![forbid(unsafe_code)]

mod settings;

use anyhow::Result;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let _config = settings::AppConfig::from_env()?;
    Ok(())
}
