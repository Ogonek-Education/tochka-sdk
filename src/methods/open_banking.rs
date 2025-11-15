use serde::Deserialize;

use crate::{ApiVersion, Client, Customer, Error, PaginatedResponse, Service};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomerPageData {
    pub customer: Vec<Customer>,
}

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
}
