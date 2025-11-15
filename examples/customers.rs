use tochka_sdk::{Client, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Production)?;

    let list = client.get_customers_list().await?;
    println!("{:#?}", list);

    Ok(())
}
