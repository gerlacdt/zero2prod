use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration, email_client::EmailClient, startup::run, telemetry,
};

const BIND_ADDR: &str = "127.0.0.1:0";

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            telemetry::get_json_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber =
            telemetry::get_json_subscriber(subscriber_name, default_filter_level, std::io::sink);
        telemetry::init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind(BIND_ADDR).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let address = format!("127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPool::connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");

    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let server = run(listener, db_pool.clone(), email_client).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { address, db_pool }
}

pub async fn clean_db() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let mut connection = PgConnection::connect_with(&configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute("DELETE FROM subscriptions;")
        .await
        .expect("Failed to cleanup database.");
}
