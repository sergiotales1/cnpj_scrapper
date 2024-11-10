use scrapper::run;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    run().await?;
    Ok(())
}
