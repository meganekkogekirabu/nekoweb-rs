use dotenv::dotenv;
use nekoweb_rs::Client;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("NEKOWEB_API_KEY")?;
    let dir = env::args().nth(1).unwrap_or("/".to_string());
    let client = Client::new("nekoweb-rs")?.authenticate(token);
    let res = client.list(&dir).await?;
    dbg!(res);
    Ok(())
}
