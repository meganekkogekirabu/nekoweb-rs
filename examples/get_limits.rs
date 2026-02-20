use dotenv::dotenv;
use nekoweb_rs::Client;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("NEKOWEB_API_KEY")?;
    let client = Client::new("nekoweb-rs")?.authenticate(token);
    let limits = client.get_limits().await?;
    dbg!(&limits);
    Ok(())
}
