use tochka_sdk::{Customer, CustomerPageData, Data, ExternalType, PaginatedResponse, Transaction};

#[test]
fn deserialize_authorized_card_transactions() {
    let json = r#"
 {
  "Data": {
    "Transactions": [
      {
        "accountId": "40817810802000000008/044525104",
        "pan": "string",
        "dateTime": "2019-01-01T06:06:06.364+00:00",
        "Amount": {
          "amount": 1234.56,
          "currency": "RUB"
        },
        "AccountAmount": {
          "amount": 1234.56,
          "currency": "RUB"
        },
        "TerminalData": {
          "city": "Perm",
          "location": "Ekaterinburg",
          "owner": "string"
        }
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<Transaction> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.data.account_id, "40817810802000000008/044525104");
    assert_eq!(parsed.data.terminal_data.city, Some("Perm".to_string()));
}

#[test]
fn deserialize_balances_list() {
    let json = r#"
    
 {
  "Data": {
    "Customer": [
      {
        "customerCode": "300000092",
        "customerType": "Personal",
        "isResident": true,
        "taxCode": "660000000000",
        "fullName": "Индивидуальный Предприниматель Тест",
        "shortName": "ИП Тест",
        "kpp": "668501001",
        "customerOgrn": "319665800211661"
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: PaginatedResponse<CustomerPageData> = serde_json::from_str(json).unwrap();

    assert_eq!(
        parsed.data.customer[0].tax_code,
        Some("660000000000".to_string())
    );
    assert_eq!(
        parsed.data.customer[0].customer_type,
        ExternalType::Personal
    );
}

#[test]
fn deserialize_balance_info() {
    let json = r#"
    
 {
  "Data": {
    "Customer": [
      {
        "customerCode": "300000092",
        "customerType": "Personal",
        "isResident": true,
        "taxCode": "660000000000",
        "fullName": "Индивидуальный Предприниматель Тест",
        "shortName": "ИП Тест",
        "kpp": "668501001",
        "customerOgrn": "319665800211661"
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: PaginatedResponse<CustomerPageData> = serde_json::from_str(json).unwrap();

    assert_eq!(
        parsed.data.customer[0].tax_code,
        Some("660000000000".to_string())
    );
    assert_eq!(
        parsed.data.customer[0].customer_type,
        ExternalType::Personal
    );
}
