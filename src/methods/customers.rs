use crate::{
    ApiVersion, Client, Customer, CustomerPageData, Data, Error, PaginatedResponse, Service,
};

impl Client {
    /// # Метод для получения списка доступных клиентов
    ///
    /// Работа с клиентами
    pub async fn get_customers_list(&self) -> Result<PaginatedResponse<CustomerPageData>, Error> {
        self.send::<PaginatedResponse<CustomerPageData>>(self.client.get(self.url(
            Service::OpenBanking,
            ApiVersion::V1_0,
            "customers",
        )))
        .await
    }

    /// # Метод для получения списка доступных клиентов
    ///
    /// Работа с клиентами
    pub async fn get_customer_info(&self, customer_code: &str) -> Result<Data<Customer>, Error> {
        self.send::<Data<Customer>>(self.client.get(self.url(
            Service::OpenBanking,
            ApiVersion::V1_0,
            format!("customers/{customer_code}").as_str(),
        )))
        .await
    }
}
