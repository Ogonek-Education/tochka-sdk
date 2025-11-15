use crate::{ApiVersion, Client, Error, PaginatedResponse, PaymentOperation, Service};

impl Client {
    pub async fn payment_operation_list(
        &self,
    ) -> Result<PaginatedResponse<Vec<PaymentOperation>>, Error> {
        self.send::<PaginatedResponse<Vec<PaymentOperation>>>(self.client.get(self.url(
            Service::Acquiring,
            ApiVersion::V1_0,
            "payments",
        )))
        .await
    }
    pub fn create_payment_operation(&self) {}
    pub fn payment_operation_info(&self) {}
}
