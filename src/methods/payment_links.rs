use crate::{
    ApiVersion, Client, CreatePaymentPayload, Data, Error, PaginatedResponse, PayloadWrapper,
    PaymentListQuery, PaymentOperation, PaymentPageData, Service,
};

impl Client {
    pub async fn payment_operation_list(
        &self,
        query: PaymentListQuery,
    ) -> Result<PaginatedResponse<PaymentPageData>, Error> {
        self.send::<PaginatedResponse<PaymentPageData>>(
            self.client
                .get(self.url(Service::Acquiring, ApiVersion::V1_0, "payments"))
                .query(&query),
        )
        .await
    }
    pub async fn create_payment_operation(
        &self,
        payload: CreatePaymentPayload,
    ) -> Result<Data<PaymentOperation>, Error> {
        self.send::<Data<PaymentOperation>>(
            self.client
                .post(self.url(Service::Acquiring, ApiVersion::V1_0, "payments"))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }
    pub async fn payment_operation_info(
        &self,
        operation_id: &str,
    ) -> Result<Data<PaymentPageData>, Error> {
        self.send::<Data<PaymentPageData>>(self.client.get(self.url(
            Service::Acquiring,
            ApiVersion::V1_0,
            format!("payments/{operation_id}").as_str(),
        )))
        .await
    }
}
