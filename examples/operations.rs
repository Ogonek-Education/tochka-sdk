use std::io::{Read, stdin};
use tochka_sdk::{Client, CreatePaymentPayload, Environment, PaymentListQuery, PaymentMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let customer_code = std::env::var("CUSTOMER_CODE").unwrap();
    let client = Client::new(Environment::Production)?;

    // Schritt 1: Zahlung erstellen
    let create = client
        .create_payment_operation(
            CreatePaymentPayload::new(10.0, &customer_code, "Оплата услуг")
                .payment_mode(PaymentMode::Card)
                .payment_mode(PaymentMode::Sbp)
                .ttl(10800),
        )
        .await?;

    println!("Zahlung erstellt:");
    println!("{:#?}", create);
    println!();
    println!("Öffne den Link, führe die Zahlung durch und drücke anschließend ENTER…");

    // Warten auf Benutzereingabe
    let mut buf = [0u8; 1];
    let _ = stdin().read(&mut buf)?;

    // Schritt 2: aktualisierte Info abfragen
    let operation = client
        .payment_operation_info(&create.data.operation_id)
        .await?;

    println!("Aktueller Status:");
    println!("{:#?}", operation);

    println!();
    println!("ENTER drücken, um die Liste aller Operationen abzurufen…");
    let _ = stdin().read(&mut buf)?;

    // Schritt 3: Liste abrufen
    let list = client
        .payment_operation_list(PaymentListQuery::new(&customer_code))
        .await?;

    println!("Operationen:");
    println!("{:#?}", list);

    Ok(())
}
