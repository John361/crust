use serde::Deserialize;

use crate::notifier::email::Email;

#[derive(Debug, Deserialize)]
pub struct Notifiers {
    email: Option<Email>,
}

impl Notifiers {
    pub async fn notify(&self, title: String, body: String) -> anyhow::Result<()> {
        if let Some(email) = &self.email {
            email.send(title.clone(), body.clone()).await?
        }

        Ok(())
    }
}
