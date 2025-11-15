use tochka_sdk::{Client, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Sandbox)?;

    let list = client.payment_operation_list().await?;
    println!("{:#?}", list);

    Ok(())
}
