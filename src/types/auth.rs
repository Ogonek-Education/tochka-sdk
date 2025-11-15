use strum::{Display, EnumString};

/// OAuth 2.0 Permissions
#[derive(EnumString, PartialEq, Display)]
pub enum Scope {
    ReadAccountsBasic,
    ReadAccountsDetail,
    ReadBalances,
    ReadStatements,
    ReadCustomerData,
    ReadSBPData,
    EditSBPData,
    CreatePaymentForSign,
    CreatePaymentOrder,
    ReadAcquiringData,
    MakeAcquiringOperation,
    ManageInvoiceData,
    ManageWebhookData,
}
