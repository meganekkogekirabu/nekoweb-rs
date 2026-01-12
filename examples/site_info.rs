use nekoweb_rs::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("nekoweb-rs")?;
    let site_info = client.get_site("test.nekoweb.org").await?;
    dbg!(site_info);
    Ok(())
}
