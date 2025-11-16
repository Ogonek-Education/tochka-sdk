use crate::{Account, AccountPageData, ApiVersion, Client, Data, Error, Service};

impl Client {
    /// Метод для получения списка доступных счетов
    pub async fn get_accounts_list(&self) -> Result<Data<AccountPageData>, Error> {
        self.send(
            self.client
                .get(self.url(Service::OpenBanking, ApiVersion::V1_0, "accounts")),
        )
        .await
    }

    /// Метод для получения информации по конкретному счёту
    pub async fn get_account_into(&self, account_id: &str) -> Result<Data<Account>, Error> {
        self.send(self.client.get(self.url(
            Service::OpenBanking,
            ApiVersion::V1_0,
            format!("accounts/{account_id}").as_str(),
        )))
        .await
    }
}
