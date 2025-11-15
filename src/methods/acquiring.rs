use crate::{AcquiringPaymentOperationInfo, Client, Error};

impl Client {
    pub fn payment_operation_list(&self) -> Result<Vec<PaymentOperation>, Error> {}
    pub fn create_payment_operation(&self) {}
    pub fn payment_operation_info(&self) {}
}
