use tochka_sdk::{Client, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new(Environment::Production)?;

    let operation_id = std::env::var("OPERATION_ID").unwrap();

    let operation = client.payment_operation_info(&operation_id).await?;

    println!("Aktueller Status:");
    println!("{:#?}", operation.data.operation[0].status);

    Ok(())
}
