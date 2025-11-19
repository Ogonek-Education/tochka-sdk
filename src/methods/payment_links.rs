use crate::{
    ApiVersion, Client, CreatePaymentPayload, Data, Error, PaginatedResponse, PayloadWrapper,
    PaymentListQuery, PaymentOperation, PaymentPageData, PaymentPath, PaymentRegistryQuery, Refund,
    RefundPayload, RegistryPageData, ResultBody, RetailerPageData, Service,
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

    /// Метод для создания ссылки на оплату
    ///
    /// Path позволяет генерить чек, если он нужен. Для этого передйте PaymentPath с нужным параметром
    pub async fn create_payment_operation(
        &self,
        payload: CreatePaymentPayload,
        path: PaymentPath,
    ) -> Result<Data<PaymentOperation>, Error> {
        if payload.customer_code.is_none() {
            return Err(Error::Api(String::from(
                "Нет customer_code. Используйте resolve_customer_code в вашем коде",
            )));
        }
        self.send::<Data<PaymentOperation>>(
            self.client
                .post(self.url(Service::Acquiring, ApiVersion::V1_0, path.as_str()))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }
    pub async fn payment_operation_info(
        &self,
        operation_id: impl Into<String>,
    ) -> Result<Data<PaymentPageData>, Error> {
        self.send::<Data<PaymentPageData>>(self.client.get(self.url(
            Service::Acquiring,
            ApiVersion::V1_0,
            format!("payments/{0}", operation_id.into()).as_str(),
        )))
        .await
    }

    /// Метод для списания средств при двухэтапной оплате
    pub async fn capture_payment(&self, operation_id: &str) -> Result<Data<ResultBody>, Error> {
        self.send::<Data<ResultBody>>(self.client.post(self.url(
            Service::Acquiring,
            ApiVersion::V1_0,
            format!("payments/{operation_id}/capture").as_str(),
        )))
        .await
    }

    pub async fn refund_payment_operation(
        &self,
        operation_id: impl Into<String>,
        payload: RefundPayload,
    ) -> Result<Data<Refund>, Error> {
        self.send(
            self.client
                .post(self.url(
                    Service::Acquiring,
                    ApiVersion::V1_0,
                    format!("payments/{0}/refund", operation_id.into()).as_str(),
                ))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для получения реестра платежей по интернет-эквайрингу
    pub async fn get_payment_registry(
        &self,
        query: PaymentRegistryQuery,
    ) -> Result<Data<RegistryPageData>, Error> {
        self.send(
            self.client
                .get(self.url(Service::Acquiring, ApiVersion::V1_0, "registry"))
                .query(&query),
        )
        .await
    }

    /// Метод для получения информации о ретейлере
    ///- *NEW* - Ретейлер создан
    ///- *ADDRESS_DADATA* - Адрес уточнен
    ///- *OPEN_ACCOUNT* - Счёт открыт
    ///- *TWPG_SENDED* - Данные мерчанта и терминала отправлены в процессинг
    ///- *RETAILER_CREATED* - Мерчант создан в процессинге
    ///- *TERMINAL_CREATED* - Терминал создан в процессинге
    ///- *FILE_SENT* - файл отправлен в НСПК
    ///- *REG* - Зарегистрирован
    ///- *CLOSE* - Закрыт
    pub async fn get_retailers(
        &self,
        customer_code: &str,
    ) -> Result<Data<RetailerPageData>, Error> {
        self.send(
            self.client
                .get(self.url(Service::Acquiring, ApiVersion::V1_0, "retailers"))
                .query(&customer_code),
        )
        .await
    }
}
