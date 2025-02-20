//! Common code for integration tests

use std::env;

use anyhow::Error;
use maik::MockServer;
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};

/// Starts a new PostgreSQL container and sets DATABASE_URL in the environment
pub async fn create_database() -> Result<(), Error> {
    let container = postgres::Postgres::default().start().await?;
    let host_port = container.get_host_port_ipv4(5432).await?;
    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        host_port
    );

    env::set_var("DATABASE_URL", connection_string);

    Ok(())
}

/// Starts a mock SMTP relay server
pub async fn create_smtp_relay() -> Result<(), Error> {
    // TODO: Implement this function
    Ok(())
}
