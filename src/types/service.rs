#[derive(Debug, Clone, Copy)]
pub enum Service {
    OpenBanking,
    Payment,
    Acquiring,
    Invoice,
    Consent,
    Sbp,
    Webhook,
}

impl Service {
    pub fn path(&self) -> &'static str {
        match self {
            Service::OpenBanking => "open-banking",
            Service::Payment => "payment",
            Service::Acquiring => "acquiring",
            Service::Invoice => "invoice",
            Service::Consent => "consent",
            Service::Sbp => "sbp",
            Service::Webhook => "webhook",
        }
    }
}
