use fusion_mail::{configuration::{get_configuration, Settings}, telemetry::init_tracing};
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use once_cell::sync::Lazy;

static TRACING: Lazy<()> = Lazy::new(|| {
        init_tracing().expect("Failed to init tracing system.");
});

#[tokio::test]
async fn health_check_works() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgPool::connect` - it is not an inherent method of the struct!
    let connection = PgPool::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let addr = spawn_app(connection);
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app(connection: PgPool) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random addr.");
    Lazy::force(&TRACING);
    let port = listener
        .local_addr()
        .expect("Failed to get the local addr.")
        .port();
    let server = fusion_mail::run(listener, connection).ok().unwrap();
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

async fn get_database_pool(config: Settings) -> PgPool {
    let db_url = config.database.connection_string_without_db();
    let mut connection = PgConnection::connect(&db_url.expose_secret())
        .await
        .expect("Failed to connect to database.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database.database_name).as_str())
        .await
        .expect("Failed to create a new database.");

    let connection_pool = PgPool::connect((&config.database.connection_string()).expose_secret().as_str())
        .await
        .expect("Failed to Connect to database.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate data.");

    connection_pool
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection = get_database_pool(configuration).await;

    let app_address = spawn_app(connection.clone());
    let client = reqwest::Client::new();
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
// Assert
async fn subscribe_returns_a_400_when_data_is_missing() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgPool::connect` - it is not an inherent method of the struct!
    let connection = PgPool::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    // Arrange
    let app_address = spawn_app(connection);
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
