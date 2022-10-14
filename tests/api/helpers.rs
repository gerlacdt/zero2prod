use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use wiremock::MockServer;
use zero2prod::{
    configuration::get_configuration, startup::get_connection_pool, startup::Application, telemetry,
};

// const BIND_ADDR: &str = "127.0.0.1:0";

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
    pub email_server: MockServer,
    pub port: u16,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
        port: application_port,
    }
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
