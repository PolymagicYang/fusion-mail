use std::net::TcpListener;

use fusion_mail::configuration::get_configuration;
use fusion_mail::run;
use sqlx::PgPool;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connection to database.");

    let config = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind a random addr.");
    run(listener, connection)?.await
}
