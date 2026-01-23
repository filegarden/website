//! See [`verify`].

use std::sync::LazyLock;

use serde_json::{Value, json};

/// The mailbox automated emails are sent from.
static SECRET_KEY: LazyLock<String> = LazyLock::new(|| {
    dotenvy::var("TURNSTILE_SECRET_KEY")
        .expect("environment variable `TURNSTILE_SECRET_KEY` should be a valid string")
});

/// The client for connecting to the CAPTCHA verification API.
static CAPTCHA_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

/// Returns whether a Cloudflare Turnstile token is valid.
///
/// # Errors
///
/// Returns an error if the verification request fails or cannot be processed.
pub(crate) async fn verify(token: &str) -> Result<bool, reqwest::Error> {
    let outcome: Value = CAPTCHA_CLIENT
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .json(&json!({
            "secret": *SECRET_KEY,
            "response": token,
        }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(outcome["success"] == true)
}
