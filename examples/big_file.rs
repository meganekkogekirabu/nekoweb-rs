use dotenv::dotenv;
use nekoweb_rs::Client;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("NEKOWEB_API_KEY")?;
    let stream = tokio::fs::File::open("examples/sample.txt").await?;
    let client = Client::new("nekoweb-rs")?.authenticate(token);
    let res = client.upload_stream("sample.txt", stream).await?;
    println!("{}", res.text().await?);
    Ok(())
}
