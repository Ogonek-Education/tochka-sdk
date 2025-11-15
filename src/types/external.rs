use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalTransationTypeEnum {
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
pub enum ExternalAcquiringPaymentTypeEnum {
    #[serde(rename = "sbp")]
    Sbp,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "tinkoff")]
    Tinkoff,
    #[serde(rename = "dolyame")]
    Dolyame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalBalanceStaticTypeEnum {
    #[serde(rename = "OpeningAvailable")]
    OpeningAvailable,
    #[serde(rename = "ClosingAvailable")]
    ClosingAvailable,
    #[serde(rename = "Expected")]
    Expected,
    #[serde(rename = "OverdraftAvailable")]
    OverdraftAvailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalBalanceTypeEnum {
    #[serde(rename = "Credit")]
    Credit,
    #[serde(rename = "Debit")]
    Debit,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalCreditDebitIndicatorEnum {
    #[serde(rename = "Credit")]
    Credit,
    #[serde(rename = "Debit")]
    Debit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalTransactionStatusEnum {
    #[serde(rename = "Booked")]
    Booked,
    #[serde(rename = "Pending")]
    Pending,
}
