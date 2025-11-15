use anyhow::Result;
use tochka_sdk::{ApiVersion, Client, Environment, Service};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(Environment::Sandbox)?;

    let response = client.get_accounts().await?;

    println!("{:#?}", response);
    Ok(())
}
