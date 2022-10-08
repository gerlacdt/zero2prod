use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_json_subscriber, get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("info".into());
        init_subscriber(subscriber);
    } else {
        let subscriber = get_json_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
        init_subscriber(subscriber);
    }

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let address = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;

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

    run(address, connection_pool, email_client)?.await
}
