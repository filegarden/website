//! Utilities for sending emails.

use std::{env::VarError, sync::LazyLock};

use askama::Template;
use async_trait::async_trait;
use html2text::render::text_renderer::TrivialDecorator;
use lettre::{
    address::Envelope,
    message::{Mailbox, MultiPart},
    transport::smtp::{authentication::Credentials, extension::ClientId},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::WEBSITE_ORIGIN;

/// An email template asking a user to verify their email.
#[derive(Template, Debug)]
#[template(path = "email/verification.html")]
pub(crate) struct VerificationMessage<'a> {
    /// The email address being verified.
    pub(crate) email: &'a str,

    /// The URL the user must visit to verify their email.
    pub(crate) verification_url: &'a str,
}

impl MessageTemplate for VerificationMessage<'_> {
    fn subject(&self) -> String {
        "Verify your email".into()
    }
}

/// An email template informing a user that someone tried to sign up with their email despite them
/// already having an account.
#[derive(Template, Debug)]
#[template(path = "email/email_taken.html")]
pub(crate) struct EmailTakenMessage<'a> {
    /// The email address used to try to sign up.
    pub(crate) email: &'a str,
}

impl MessageTemplate for EmailTakenMessage<'_> {
    fn subject(&self) -> String {
        "Sign-up failed for existing account".into()
    }
}

/// An email template giving a user a link to reset their password.
#[derive(Template, Debug)]
#[template(path = "email/password_reset.html")]
pub(crate) struct PasswordResetMessage<'a> {
    /// The email address that the password reset was submitted with.
    pub(crate) email: &'a str,

    /// The URL the user must visit to reset their password.
    pub(crate) password_reset_url: &'a str,
}

impl MessageTemplate for PasswordResetMessage<'_> {
    fn subject(&self) -> String {
        "Reset your password?".into()
    }
}

/// An email template informing a user that someone tried to reset a password for their email
/// despite them not having an account.
#[derive(Template, Debug)]
#[template(path = "email/password_reset_failed.html")]
pub(crate) struct PasswordResetFailedMessage<'a> {
    /// The email address that the password reset was submitted with.
    pub(crate) email: &'a str,
}

impl MessageTemplate for PasswordResetFailedMessage<'_> {
    fn subject(&self) -> String {
        "Password reset failed".into()
    }
}

/// The mailbox automated emails are sent from.
static FROM_MAILBOX: LazyLock<Mailbox> = LazyLock::new(|| {
    dotenvy::var("FROM_MAILBOX")
        .expect("environment variable `FROM_MAILBOX` should be a valid string")
        .parse()
        .expect("environment variable `FROM_MAILBOX` should be a valid mailbox")
});

/// An HTML [`Template`] for an email message.
pub(crate) trait MessageTemplate: Template {
    /// Gets the message's subject line.
    fn subject(&self) -> String;

    /// Generates a subject and multipart HTML and plain text body for the email message template.
    fn to(&self, mailbox: Mailbox) -> Message {
        let mut subject = self.subject();
        subject.push_str(" | File Garden");

        let html = self.to_string();
        let plain = html2text::config::with_decorator(TrivialDecorator::new())
            .string_from_read(html.as_bytes(), usize::MAX)
            .expect("message HTML should be convertible to text");

        Message::builder()
            .from(FROM_MAILBOX.clone())
            .to(mailbox)
            .subject(subject)
            .multipart(MultiPart::alternative_plain_html(plain, html))
            .expect("message should be valid")
    }
}

/// The SMTP transport used to send automated emails.
static MAILER: LazyLock<AsyncOptionalSmtpTransport> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        return AsyncOptionalSmtpTransport::Debug;
    }

    let hostname = dotenvy::var("SMTP_HOSTNAME")
        .expect("environment variable `SMTP_HOSTNAME` should be a valid string");
    let username = dotenvy::var("SMTP_USERNAME")
        .expect("environment variable `SMTP_USERNAME` should be a valid string");
    let password = dotenvy::var("SMTP_PASSWORD")
        .expect("environment variable `SMTP_PASSWORD` should be a valid string");

    let mut smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&hostname)
        .expect("SMTP relay couldn't be initialized")
        .credentials(Credentials::new(username, password));

    match dotenvy::var("SMTP_HELO_DOMAIN") {
        // If the environment variable is unset, let `lettre` default to using the OS hostname.
        Err(dotenvy::Error::EnvVar(VarError::NotPresent)) => {}

        helo_domain => {
            let helo_domain = helo_domain
                .expect("environment variable `SMTP_HELO_DOMAIN` should be a valid string if set");

            smtp_transport = smtp_transport.hello_name(ClientId::Domain(helo_domain));
        }
    }

    AsyncOptionalSmtpTransport::Smtp(smtp_transport.build())
});

/// An async mail transport with the option to either send mail over SMTP or simply print all sent
/// messages.
enum AsyncOptionalSmtpTransport {
    /// An async mail transport that prints all sent messages.
    Debug,

    /// An async mail transport that sends messages over SMTP.
    Smtp(AsyncSmtpTransport<Tokio1Executor>),
}

#[async_trait]
impl AsyncTransport for AsyncOptionalSmtpTransport {
    type Ok = ();
    type Error = <AsyncSmtpTransport<Tokio1Executor> as AsyncTransport>::Error;

    async fn send_raw(&self, envelope: &Envelope, email: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Debug => {
                println!(
                    "======== DEBUG MAIL START ========\n{}\n======== DEBUG MAIL END ========",
                    String::from_utf8_lossy(email),
                );
            }
            Self::Smtp(smtp_transport) => {
                smtp_transport.send_raw(envelope, email).await?;
            }
        }
        Ok(())
    }
}

/// A trait for sending messages using the SMTP configuration from `.env`.
pub(crate) trait SendMessage {
    /// Sends the message in the background.
    ///
    /// Errors are ignored so they can't propagate to end users. Otherwise, users could tell if an
    /// email sent successfully or not, which can allow for user enumeration in some circumstances.
    fn send(self);
}

impl SendMessage for Message {
    fn send(self) {
        tokio::spawn(MAILER.send(self));
    }
}
