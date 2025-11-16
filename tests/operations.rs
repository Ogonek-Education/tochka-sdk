use tochka_sdk::{Customer, CustomerPageData, Data, ExternalType, PaginatedResponse};

#[test]
fn serialize_create_payment_payload() {
    let json = r#"
   {
  "Data": {
    "customerCode": "300000092",
    "amount": "1234.00",
    "purpose": "Перевод за оказанные услуги",
    "redirectUrl": "https://example.com",
    "failRedirectUrl": "https://example.com/fail",
    "paymentMode": [
      "sbp",
      "card",
      "tinkoff",
      "dolyame"
    ],
    "saveCard": true,
    "consumerId": "fedac807-078d-45ac-a43b-5c01c57edbf8",
    "merchantId": "200000000001056",
    "preAuthorization": true,
    "ttl": 10080,
    "paymentLinkId": "string"
  }
}
    "#;

    let parsed: Data<Customer> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.data.customer_code, "300000092");
    assert_eq!(parsed.data.customer_type, ExternalType::Personal);
}

#[test]
fn deserialize_customer_list() {
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
