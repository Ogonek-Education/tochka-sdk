use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalAccountStatusEnum {
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Deleted")]
    Deleted,
    #[serde(rename = "ProForma")]
    ProForma,
    #[serde(rename = "Pending")]
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalAccountSubTypeEnum {
    #[serde(rename = "CreditCard")]
    CreditCard,
    #[serde(rename = "CurrentAccount")]
    CurrentAccount,
    #[serde(rename = "Loan")]
    Loan,
    #[serde(rename = "Mortgage")]
    Mortgage,
    #[serde(rename = "PrePaidCard")]
    PrePaidCard,
    #[serde(rename = "Savings")]
    Savings,
    #[serde(rename = "Special")]
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalConsentTypeEnum {
    #[serde(rename = "ReadAccountsBasic")]
    ReadAccountsBasic,
    #[serde(rename = "ReadAccountsDetail")]
    ReadAccountsDetail,
    #[serde(rename = "ReadBalances")]
    ReadBalances,
    #[serde(rename = "ReadStatements")]
    ReadStatements,
    #[serde(rename = "ReadTransactionsBasic")]
    ReadTransactionsBasic,
    #[serde(rename = "ReadTransactionsCredits")]
    ReadTransactionsCredits,
    #[serde(rename = "ReadTransactionsDebits")]
    ReadTransactionsDebits,
    #[serde(rename = "ReadTransactionsDetail")]
    ReadTransactionsDetail,
    #[serde(rename = "ReadCustomerData")]
    ReadCustomerData,
    #[serde(rename = "ReadSBPData")]
    ReadSbpData,
    #[serde(rename = "EditSBPData")]
    EditSbpData,
    #[serde(rename = "CreatePaymentForSign")]
    CreatePaymentForSign,
    #[serde(rename = "CreatePaymentOrder")]
    CreatePaymentOrder,
    #[serde(rename = "ReadAcquiringData")]
    ReadAcquiringData,
    #[serde(rename = "MakeAcquiringOperation")]
    MakeAcquiringOperation,
    #[serde(rename = "ManageInvoiceData")]
    ManageInvoiceData,
    #[serde(rename = "ManageWebhookData")]
    ManageWebhookData,
    #[serde(rename = "MakeCustomer")]
    MakeCustomer,
    #[serde(rename = "ManageGuarantee")]
    ManageGuarantee,
}
