use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::configure_tracing;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    configure_tracing();
    // let subscriber = get_subscriber("zero2prod".into(), "info".into());
    // init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    run(address, connection_pool)?.await
}
