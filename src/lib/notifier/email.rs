use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Email {
    host: String,
    port: u16,
    username: String,
    password: String,
    from: String,
    to: String,
}

impl Email {
    pub async fn send(&self, subject: String, content: String) -> anyhow::Result<()> {
        let username = self.username.to_string();
        let password = self.password.to_string();
        let from = self.from.to_string();
        let to = self.to.to_string();

        let creds = Credentials::new(username, password);
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)?
            .port(self.port)
            .credentials(creds)
            .build();

        let email = Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject.clone())
            .header(ContentType::TEXT_PLAIN)
            .body(content)?;

        mailer.send(email).await?;
        Ok(())
    }
}
