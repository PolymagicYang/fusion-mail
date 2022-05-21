use std::net::TcpListener;

use fusion_mail::configuration::get_configuration;
use fusion_mail::run;
use fusion_mail::telemetry::init_tracing;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing().expect("Failed to init tracing system.");
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connection to database.");

    let config = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("{}:{}", config.application.host, config.application.port))
        .expect("Failed to bind a random addr.");
    run(listener, connection)?.await
}
