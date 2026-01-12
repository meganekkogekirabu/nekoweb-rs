use dotenv::dotenv;
use nekoweb_rs::Client;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let api_key = env::var("NEKOWEB_API_KEY")?;
    let client = Client::new("nekoweb-rs")?.authenticate(api_key);
    let response = client.create_file("test").await?;
    println!("{}", response.text().await?);
    Ok(())
}
