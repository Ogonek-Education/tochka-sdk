use crate::{CashAccount, Contractor, ContractorBank, CreditDebitIndicator, TaxFields};
use chrono::{DateTime, NaiveDate, Utc};
use codes_iso_4217::CurrencyCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub account_id: String,
    pub pan: String,
    pub date_time: DateTime<Utc>,
    #[serde(rename = "Amount")]
    pub amount: Amount,
    #[serde(rename = "AccountAmount")]
    pub account_amount: AccountAmount,
    #[serde(rename = "TerminalData")]
    pub terminal_data: TerminalData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub amount: f64,
    pub amount_nat: Option<u32>,
    pub currency: CurrencyCode,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountAmount {
    pub amount: f64,
    pub currency: CurrencyCode,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TerminalData {
    pub city: Option<String>,
    pub location: Option<String>,
    pub owner: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionPageData {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionStatus {
    Booked,
    Pending,
}

#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatement {
    pub transaction_id: Option<String>,
    pub payment_id: Option<String>,
    pub credit_debit_indicator: CreditDebitIndicator,
    pub status: TransactionStatus,
    pub document_number: Option<String>,
    pub transaction_type_code: Option<TransationTypeCode>,
    pub document_process_date: Option<NaiveDate>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub subfields: TransactionSubfields,
}

#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionSubfields {
    amount: Amount,
    debtor_party: Contractor,
    debtor_account: CashAccount,
    debtor_agent: ContractorBank,

    creditor_party: Contractor,
    creditor_account: CashAccount,
    creditor_agent: ContractorBank,

    tax_fields: TaxFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransationTypeCode {
    #[serde(rename = "Неопределенное значение")]
    Undefined,

    #[serde(rename = "Платежное поручение")]
    PaymentOrder,

    #[serde(rename = "Платежное требование")]
    PaymentRequest,

    #[serde(rename = "Денежный чек, РКО")]
    CashCheckRko,

    #[serde(rename = "Объявление на взнос наличными, ПКО")]
    CashDepositPko,

    #[serde(rename = "Требование-поручение")]
    DemandOrder,

    #[serde(rename = "Инкассовое поручение")]
    CollectionOrder,

    #[serde(rename = "Расчетный чек")]
    SettlementCheck,

    #[serde(rename = "Аккредитив")]
    LetterOfCredit,

    #[serde(rename = "Мемориальный ордер")]
    MemorialOrder,

    #[serde(rename = "Погашение кредита")]
    LoanRepayment,

    #[serde(rename = "Выдача кредита")]
    LoanIssuance,

    #[serde(rename = "Авизо")]
    Aviso,

    #[serde(rename = "Банковские карты")]
    BankCards,

    #[serde(rename = "Платежный ордер")]
    PaymentInstruction,

    #[serde(rename = "Банковский ордер")]
    BankOrder,

    #[serde(rename = "Ордер по передаче ценностей")]
    AssetTransferOrder,

    #[serde(rename = "Программный ордер")]
    ProgramOrder,

    #[serde(rename = "Импортированная запись")]
    ImportedRecord,
}
