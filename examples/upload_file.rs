use dotenv::dotenv;
use nekoweb_rs::Client;
use std::{env, fs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let api_key = env::var("NEKOWEB_API_KEY")?;
    let client = Client::new("nekoweb-rs")?.authenticate(api_key);
    let buffer = fs::read("examples/sample.txt")?;
    let response = client.upload_file("sample.txt", buffer).await?;
    println!("{}", response.text().await?);
    Ok(())
}
