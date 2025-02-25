//! File Garden's backend web server.

use std::sync::LazyLock;

use axum::handler::Handler;
use tokio::net::TcpListener;

pub mod api;
mod content;
mod crypto;
mod db;
mod email;
pub mod id;
mod percent_encoding;
mod response;
mod router;
mod website;

/// The URI origin for user-uploaded content.
pub(crate) static CONTENT_ORIGIN: LazyLock<String> = LazyLock::new(|| {
    dotenvy::var("CONTENT_ORIGIN")
        .expect("environment variable `CONTENT_ORIGIN` should be a valid string")
});

/// The URI origin for the website.
pub(crate) static WEBSITE_ORIGIN: LazyLock<String> = LazyLock::new(|| {
    dotenvy::var("WEBSITE_ORIGIN")
        .expect("environment variable `WEBSITE_ORIGIN` should be a valid string")
});

/// The state passed to all of the routes.
#[derive(Clone, Debug)]
pub struct AppState {
    /// The database pool shared between all routes.
    db_pool: sqlx::PgPool,
}

/// # Errors
///
/// See implementation.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Loading environment variables...");

    let db_url = dotenvy::var("DATABASE_URL")?;
    let address = dotenvy::var("ADDRESS")?;

    println!("Initializing database...");

    let db_pool = db::initialize(&db_url).await?;

    println!("Listening to {address}...");

    let listener = TcpListener::bind(address).await?;

    println!("Ready!");

    axum::serve(
        listener,
        router::handle
            .with_state(AppState { db_pool })
            .into_make_service(),
    )
    .await?;

    Ok(())
}
