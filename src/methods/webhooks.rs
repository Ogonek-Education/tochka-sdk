use crate::{Client, Data, Error, ResultBody, Service, Webhook, WebhookType};

impl Client {
    /// Метод для создания вебхуков
    ///
    /// почему-то метод put
    pub async fn create_webhook(&self, payload: Webhook) -> Result<Data<Webhook>, Error> {
        let id = self
            .client_id
            .as_deref()
            .expect("client_id must be set before create_webhook()");

        self.send::<Data<Webhook>>(
            self.client
                .put(self.url(Service::Webhook, crate::ApiVersion::V1_0, id))
                .json(&payload),
        )
        .await
    }
    /// Метод для изменения URL и типа вебхука
    ///
    /// почему-то это пост
    pub async fn edit_webhook(&self, payload: Webhook) -> Result<Data<Webhook>, Error> {
        let id = self
            .client_id
            .as_deref()
            .expect("client_id must be set before edit_webhook()");
        self.send::<Data<Webhook>>(
            self.client
                .post(self.url(Service::Webhook, crate::ApiVersion::V1_0, id))
                .json(&payload),
        )
        .await
    }
    /// Метод для получения списка вебхуков приложения
    pub async fn get_webhooks(&self) -> Result<Data<Webhook>, Error> {
        let id = self
            .client_id
            .as_deref()
            .expect("client_id must be set before get_webhooks()");
        self.send::<Data<Webhook>>(self.client.get(self.url(
            Service::Webhook,
            crate::ApiVersion::V1_0,
            id,
        )))
        .await
    }
    /// Метод для удаления вебхука
    pub async fn delete_webhook(&self) -> Result<Data<ResultBody>, Error> {
        let id = self
            .client_id
            .as_deref()
            .expect("client_id must be set before delete_webhook()");
        self.send::<Data<ResultBody>>(self.client.delete(self.url(
            Service::Webhook,
            crate::ApiVersion::V1_0,
            id,
        )))
        .await
    }
    /// Метод для проверки отпраки хука
    pub async fn send_webhook(&self, payload: WebhookType) -> Result<Data<ResultBody>, Error> {
        let id = self
            .client_id
            .as_deref()
            .expect("client_id must be set before send_webhook()");
        self.send::<Data<ResultBody>>(
            self.client
                .post(self.url(
                    Service::Webhook,
                    crate::ApiVersion::V1_0,
                    format!("{0}/test_send", id).as_str(),
                ))
                .json(&payload),
        )
        .await
    }
}
