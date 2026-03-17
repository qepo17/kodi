mod clients;
mod models;
mod tools;

use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("OPENROUTER_API_KEY")?;
    let client = reqwest::Client::new();

    let query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Hello!".to_string());

    clients::agent_loop(&client, &api_key, &query).await?;

    Ok(())
}
